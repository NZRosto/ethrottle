//! Basic tester for MC33HB2001.

#![no_std]
#![no_main]

use core::convert::Infallible;

use embedded_hal::digital::OutputPin;
use embedded_hal::pwm::SetDutyCycle;
use embedded_hal_async::spi::SpiDevice;
use synch::RateExt;
use woven::RaceSame;
use {defmt_rtt as _, panic_probe as _};

const ARM_FREQUENCY: u32 = 48_000_000;

type Timer = synch::Timer<ARM_FREQUENCY>;

#[hal::entry]
fn entry() -> ! {
    cassette::block_on(main())
}

async fn main() -> ! {
    defmt::info!("Hello World!");

    let hal::Peripherals {
        pins,
        tpm2,
        dma0,
        dma1,
        spi0,
        ..
    } = unsafe { hal::Peripherals::steal() };

    let core = unsafe { cortex_m::Peripherals::steal() };

    let timer = synch::Timer::new(core.SYST);

    let bus = hal::spi::SpiMaster::new(spi0, dma0, dma1, pins.PTD1, pins.PTD2, pins.PTD3);
    let Ok(device) = embedded_hal_bus::spi::ExclusiveDevice::new_no_delay(
        bus,
        hal::gpio::Output::new(pins.PTD0),
    );

    let mut pwm2 = hal::tpm::pwm::Pwm::new(tpm2);
    let channels2 = pwm2.split();

    let r = channels2.channel0.use_with(pins.PTB18);
    let g = channels2.channel1.use_with(pins.PTB19);

    (
        communicate(
            device,
            hal::gpio::Output::new(pins.PTA17),
            hal::gpio::Output::new(pins.PTE31),
            timer,
        ),
        cycle_leds(r, g, timer),
    )
        .race_same()
        .await
}

async fn cycle_leds(
    mut r: impl SetDutyCycle<Error = Infallible>,
    mut g: impl SetDutyCycle<Error = Infallible>,
    timer: Timer,
) -> ! {
    let mut ticker = timer.repeat_at(1.kHz());

    let mut hue: u16 = 0;
    loop {
        while hue < u16::MAX {
            ticker.next().await;
            hue = hue.saturating_add(10);

            let Ok(()) = r.set_duty_cycle(u16::MAX - hue / 8);
            let Ok(()) = g.set_duty_cycle((u16::MAX / 8) * 7 + hue / 8);

            cassette::yield_now().await; // Ensure loop not blocking executor.
        }

        while hue > u16::MIN {
            ticker.next().await;
            hue = hue.saturating_sub(10);

            let Ok(()) = r.set_duty_cycle(u16::MAX - hue / 8);
            let Ok(()) = g.set_duty_cycle((u16::MAX / 8) * 7 + hue / 8);

            cassette::yield_now().await; // Ensure loop not blocking executor.
        }
    }
}

async fn communicate(
    spi: impl SpiDevice<Error = embedded_hal_bus::spi::DeviceError<Infallible, Infallible>>,
    enable: impl OutputPin<Error = Infallible>,
    disable: impl OutputPin<Error = Infallible>,
    timer: Timer,
) -> ! {
    const BASE_CONFIG: mc33hb2001::Configuration = mc33hb2001::Configuration::new()
        .with_bridge_mode(mc33hb2001::BridgeMode::HBridge)
        .with_control_mode(mc33hb2001::ControlMode::Spi)
        .with_virtual_input_1(mc33hb2001::Input::High);

    let mut ethrottle = mc33hb2001::Driver::new(spi, enable, disable, timer)
        .await
        .unwrap();

    let mut on = 100;

    loop {
        if on >= 900 {
            on = 100;
        } else {
            on += 1;
        }

        timer.after(synch::Duration::micros(1000 - on)).await;

        let Ok(()) = ethrottle
            .set_configuration(BASE_CONFIG.with_virtual_input_2(mc33hb2001::Input::High))
            .await;

        timer.after(synch::Duration::micros(on)).await;

        let Ok(()) = ethrottle
            .set_configuration(BASE_CONFIG.with_virtual_input_2(mc33hb2001::Input::Low))
            .await;
    }
}
