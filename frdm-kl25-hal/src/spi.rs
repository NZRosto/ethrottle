#![allow(clippy::module_name_repetitions)]

use woven::Join;

use crate::dma::Dma;
use crate::pins::Pin;

pub trait Spi {
    const INDEX: u32;
    const RX_SLOT: u8;
    const TX_SLOT: u8;
}

pub struct Spi0 {
    _private: (),
}

impl Spi0 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Spi for Spi0 {
    const INDEX: u32 = 0;
    const RX_SLOT: u8 = 16;
    const TX_SLOT: u8 = 17;
}

pub struct Spi1 {
    _private: (),
}

impl Spi1 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Spi for Spi1 {
    const INDEX: u32 = 1;
    const RX_SLOT: u8 = 18;
    const TX_SLOT: u8 = 19;
}

pub trait MosiPin<S> {
    const ALT: crate::mux::Alternate;
}

pub trait MisoPin<S> {
    const ALT: crate::mux::Alternate;
}

pub trait SckPin<S> {
    const ALT: crate::mux::Alternate;
}

pub struct SpiMaster<S, T, R, SCK, MOSI, MISO> {
    spi: S,
    tx: T,
    rx: R,
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
}

impl<S, T, R, SCK, MOSI, MISO> SpiMaster<S, T, R, SCK, MOSI, MISO>
where
    S: Spi,
    T: Dma,
    R: Dma,
    SCK: SckPin<S> + Pin,
    MISO: MisoPin<S> + Pin,
    MOSI: MosiPin<S> + Pin,
{
    pub fn new(spi: S, tx_ch: T, rx_ch: R, sck: SCK, mosi: MOSI, miso: MISO) -> Self {
        let peripherals = unsafe { pac::Peripherals::steal() };

        crate::pins::enable_port_clock::<SCK>();
        crate::pins::enable_port_clock::<MISO>();
        crate::pins::enable_port_clock::<MOSI>();

        crate::mux::set_alternate::<SCK>(SCK::ALT);
        crate::mux::set_alternate::<MISO>(MISO::ALT);
        crate::mux::set_alternate::<MOSI>(MOSI::ALT);

        // Enable spi clock
        match S::INDEX {
            0 => peripherals.sim.scgc4().modify(|_, w| w.spi0()._1()),
            1 => peripherals.sim.scgc4().modify(|_, w| w.spi1()._1()),

            _ => unreachable!(),
        };

        // Set baud rate
        match S::INDEX {
            0 => peripherals
                .spi0
                .br()
                .write(|w| w.sppr()._000().spr()._0011()),
            1 => peripherals
                .spi1
                .br()
                .write(|w| w.sppr()._000().spr()._0011()),

            _ => unreachable!(),
        };

        peripherals.sim.scgc6().modify(|_, w| w.dmamux()._1());

        // Reset DMAMUX
        peripherals.dmamux0.chcfg(T::INDEX as usize).reset();
        peripherals.dmamux0.chcfg(R::INDEX as usize).reset();

        // Set slot number in DMAMUX, then enable.
        peripherals
            .dmamux0
            .chcfg(T::INDEX as usize)
            .write(|w| unsafe { w.source().bits(S::TX_SLOT) });
        peripherals
            .dmamux0
            .chcfg(R::INDEX as usize)
            .write(|w| unsafe { w.source().bits(S::RX_SLOT) });
        peripherals
            .dmamux0
            .chcfg(T::INDEX as usize)
            .modify(|_, w| w.enbl()._1());
        peripherals
            .dmamux0
            .chcfg(R::INDEX as usize)
            .modify(|_, w| w.enbl()._1());

        // Enable SPI, master mode
        match S::INDEX {
            0 => peripherals.spi0.c1().write(|w| w.spe()._1().mstr()._1()),
            1 => peripherals.spi1.c1().write(|w| w.spe()._1().mstr()._1()),

            _ => unreachable!(),
        };

        Self {
            spi,
            tx: tx_ch,
            rx: rx_ch,
            sck,
            mosi,
            miso,
        }
    }

    pub fn release(self) -> (S, T, R, SCK, MOSI, MISO) {
        (self.spi, self.tx, self.rx, self.sck, self.mosi, self.miso)
    }
}

impl<S, T, R, SCK, MOSI, MISO> SpiMaster<S, T, R, SCK, MOSI, MISO>
where
    S: Spi,
    T: Dma,
    R: Dma,
    SCK: SckPin<S> + Pin,
    MISO: MisoPin<S> + Pin,
    MOSI: MosiPin<S> + Pin,
{
    async fn transfer_inner(&self, read: &mut [u8], write: &[u8]) {
        let peripherals = unsafe { pac::Peripherals::steal() };

        let data_address = match S::INDEX {
            0 => peripherals.spi0.d().as_ptr(),
            1 => peripherals.spi1.d().as_ptr(),

            _ => unreachable!(),
        };

        unsafe {
            crate::dma::setup_dma_transfer::<T>(
                write.as_ptr(),
                data_address,
                write.len().try_into().unwrap(),
                true,
                false,
                false,
                true,
            );

            crate::dma::setup_dma_transfer::<R>(
                data_address,
                read.as_mut_ptr(),
                read.len().try_into().unwrap(),
                false,
                true,
                false,
                true,
            );
        }

        // Enable DMA in SPI block
        match S::INDEX {
            0 => peripherals
                .spi0
                .c2()
                .write(|w| w.rxdmae()._1().txdmae()._1()),
            1 => peripherals
                .spi1
                .c2()
                .write(|w| w.rxdmae()._1().txdmae()._1()),

            _ => unreachable!(),
        };

        unsafe {
            (
                crate::dma::wait_dma_transfer::<T>(),
                crate::dma::wait_dma_transfer::<R>(),
            )
                .join()
                .await;
        }
    }
}

impl<S, T, R, SCK, MOSI, MISO> embedded_hal_async::spi::ErrorType
    for SpiMaster<S, T, R, SCK, MOSI, MISO>
{
    type Error = core::convert::Infallible;
}

impl<S, T, R, SCK, MOSI, MISO> embedded_hal_async::spi::SpiBus
    for SpiMaster<S, T, R, SCK, MOSI, MISO>
where
    S: Spi,
    T: Dma,
    R: Dma,
    SCK: SckPin<S> + Pin,
    MISO: MisoPin<S> + Pin,
    MOSI: MosiPin<S> + Pin,
{
    async fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.transfer_inner(words, &[]).await;
        Ok(())
    }

    async fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.transfer_inner(&mut [], words).await;
        Ok(())
    }

    async fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.transfer_inner(read, write).await;
        Ok(())
    }

    async fn transfer_in_place(&mut self, _words: &mut [u8]) -> Result<(), Self::Error> {
        unimplemented!()
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

macro_rules! s_pin {
    ($pin:ident, $tpm:ident, $p_type:ident, $alt:ident) => {
        impl $p_type<$tpm> for crate::pins::$pin {
            const ALT: crate::mux::Alternate = crate::mux::Alternate::$alt;
        }
    };
}

s_pin!(PTE1, Spi1, MosiPin, Alt2);
s_pin!(PTE1, Spi1, MisoPin, Alt5);
s_pin!(PTE2, Spi1, SckPin, Alt2);
s_pin!(PTE3, Spi1, MisoPin, Alt2);
s_pin!(PTE3, Spi1, MosiPin, Alt5);
s_pin!(PTA15, Spi0, SckPin, Alt2);
s_pin!(PTA16, Spi0, MosiPin, Alt2);
s_pin!(PTA16, Spi0, MisoPin, Alt5);
s_pin!(PTA17, Spi0, MisoPin, Alt2);
s_pin!(PTA17, Spi0, MosiPin, Alt5);
s_pin!(PTB11, Spi1, SckPin, Alt2);
s_pin!(PTB16, Spi1, MosiPin, Alt2);
s_pin!(PTB16, Spi1, MisoPin, Alt5);
s_pin!(PTB17, Spi1, MisoPin, Alt2);
s_pin!(PTB17, Spi1, MosiPin, Alt5);
s_pin!(PTC5, Spi0, SckPin, Alt2);
s_pin!(PTC6, Spi0, MosiPin, Alt2);
s_pin!(PTC6, Spi0, MisoPin, Alt5);
s_pin!(PTC7, Spi0, MisoPin, Alt2);
s_pin!(PTC7, Spi0, MosiPin, Alt5);
s_pin!(PTD1, Spi0, SckPin, Alt2);
s_pin!(PTD2, Spi0, MosiPin, Alt2);
s_pin!(PTD2, Spi0, MisoPin, Alt5);
s_pin!(PTD3, Spi0, MisoPin, Alt2);
s_pin!(PTD3, Spi0, MosiPin, Alt5);
s_pin!(PTD5, Spi1, SckPin, Alt2);
s_pin!(PTD6, Spi1, MosiPin, Alt2);
s_pin!(PTD6, Spi1, MisoPin, Alt5);
s_pin!(PTD7, Spi1, MisoPin, Alt2);
s_pin!(PTD7, Spi1, MosiPin, Alt5);
