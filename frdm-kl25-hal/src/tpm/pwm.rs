#![allow(clippy::needless_lifetimes)]

use core::marker::PhantomData;

use super::{Timer, TimerPin, Tpm0, Tpm1, Tpm2};
use crate::pins::Pin;

pub struct Pwm<T> {
    _timer: T,
}

impl<T> Pwm<T>
where
    T: Timer,
{
    pub fn new(timer: T) -> Self {
        super::ensure_clock_active::<T>();
        super::enable_timer::<T>(false);
        super::set_timer_mod_value::<T>(u16::MAX - 1);
        match T::INDEX {
            0 => {
                super::disable_channel::<T, 0>();
                super::disable_channel::<T, 1>();
                super::disable_channel::<T, 2>();
                super::disable_channel::<T, 3>();
                super::disable_channel::<T, 4>();
                super::disable_channel::<T, 5>();
            }
            1 | 2 => {
                super::disable_channel::<T, 0>();
                super::disable_channel::<T, 1>();
            }

            _ => unreachable!(),
        }
        super::enable_timer::<T>(true);

        Self { _timer: timer }
    }
}

impl Pwm<Tpm0> {
    pub fn split<'t>(&'t mut self) -> HexChannels<'t, Tpm0> {
        HexChannels {
            channel0: Channel {
                _timer: PhantomData,
            },
            channel1: Channel {
                _timer: PhantomData,
            },
            channel2: Channel {
                _timer: PhantomData,
            },
            channel3: Channel {
                _timer: PhantomData,
            },
            channel4: Channel {
                _timer: PhantomData,
            },
            channel5: Channel {
                _timer: PhantomData,
            },
        }
    }
}

impl Pwm<Tpm1> {
    pub fn split<'t>(&'t mut self) -> DualChannels<'t, Tpm1> {
        DualChannels {
            channel0: Channel {
                _timer: PhantomData,
            },
            channel1: Channel {
                _timer: PhantomData,
            },
        }
    }
}

impl Pwm<Tpm2> {
    pub fn split<'t>(&'t mut self) -> DualChannels<'t, Tpm2> {
        DualChannels {
            channel0: Channel {
                _timer: PhantomData,
            },
            channel1: Channel {
                _timer: PhantomData,
            },
        }
    }
}

pub struct HexChannels<'t, T> {
    pub channel0: Channel<'t, T, 0>,
    pub channel1: Channel<'t, T, 1>,
    pub channel2: Channel<'t, T, 2>,
    pub channel3: Channel<'t, T, 3>,
    pub channel4: Channel<'t, T, 4>,
    pub channel5: Channel<'t, T, 5>,
}

pub struct DualChannels<'t, T> {
    pub channel0: Channel<'t, T, 0>,
    pub channel1: Channel<'t, T, 1>,
}

pub struct Channel<'t, T, const N: u32> {
    _timer: PhantomData<&'t T>,
}

impl<'t, T, const N: u32> Channel<'t, T, N>
where
    T: Timer,
{
    pub fn use_with<P>(self, pin: P) -> ActiveChannel<'t, P, T, N>
    where
        P: Pin + TimerPin<T, N>,
    {
        // Enable relevant port clock
        crate::pins::enable_port_clock::<P>();

        // Mux the pin as the required alt
        crate::mux::set_alternate::<P>(P::ALT);

        super::disable_channel::<T, N>();
        // Busy loop to wait for channel to disable
        for _ in 0..50 {
            cortex_m::asm::nop();
        }
        super::set_channel_value::<T, N>(0);
        super::configure_for_pwm::<T, N>();
        // Busy loop to wait for channel to enable
        for _ in 0..50 {
            cortex_m::asm::nop();
        }

        ActiveChannel {
            pin,
            _timer: PhantomData,
        }
    }
}

pub struct ActiveChannel<'t, P, T, const N: u32> {
    pin: P,
    _timer: PhantomData<&'t T>,
}

impl<'t, P, T, const N: u32> ActiveChannel<'t, P, T, N> {
    pub fn release(self) -> (Channel<'t, T, N>, P) {
        (
            Channel {
                _timer: PhantomData,
            },
            self.pin,
        )
    }
}

impl<'t, P, T, const N: u32> embedded_hal::pwm::ErrorType for ActiveChannel<'t, P, T, N> {
    type Error = core::convert::Infallible;
}

impl<'t, P, T, const N: u32> embedded_hal::pwm::SetDutyCycle for ActiveChannel<'t, P, T, N>
where
    T: Timer,
    P: Pin + TimerPin<T, N>,
{
    fn max_duty_cycle(&self) -> u16 {
        u16::MAX
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        super::set_channel_value::<T, N>(duty);
        Ok(())
    }
}
