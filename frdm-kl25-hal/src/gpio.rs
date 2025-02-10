use crate::pins::Pin;

pub struct Output<P> {
    _pin: P,
}

impl<P> Output<P>
where
    P: Pin,
{
    pub fn new(pin: P) -> Self {
        // Enable relevant port clock
        crate::pins::enable_port_clock::<P>();

        // Mux pin as GPIO (Alternate 1)
        crate::mux::set_alternate::<P>(crate::mux::Alternate::Gpio);

        // Set data direction to output
        set_data_direction::<P>(true);

        Self { _pin: pin }
    }

    #[allow(clippy::unused_self)]
    fn set(&mut self, set: bool) {
        set_data_output::<P>(set);
    }
}

pub struct Input<P> {
    _pin: P,
}

impl<P> Input<P>
where
    P: Pin,
{
    pub fn new(pin: P) -> Self {
        // Enable relevant port clock
        crate::pins::enable_port_clock::<P>();

        // Mux pin as GPIO (Alternate 1)
        crate::mux::set_alternate::<P>(crate::mux::Alternate::Gpio);

        // Set data direction to input
        set_data_direction::<P>(false);

        Self { _pin: pin }
    }

    #[allow(clippy::unused_self)]
    fn get(&mut self) -> bool {
        data_input::<P>()
    }
}

impl<P> embedded_hal::digital::ErrorType for Output<P> {
    type Error = core::convert::Infallible;
}

impl<P> embedded_hal::digital::OutputPin for Output<P>
where
    P: Pin,
{
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set(false);
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set(true);
        Ok(())
    }
}

impl<P> embedded_hal::digital::ErrorType for Input<P> {
    type Error = core::convert::Infallible;
}

impl<P> embedded_hal::digital::InputPin for Input<P>
where
    P: Pin,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.get())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.get())
    }
}

fn set_data_direction<P: Pin>(out: bool) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    let bit = u32::from(out);

    match P::PORT {
        0 => peripherals
            .gpioa
            .pddr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (bit << P::INDEX)) }),
        1 => peripherals
            .gpiob
            .pddr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (bit << P::INDEX)) }),
        2 => peripherals
            .gpioc
            .pddr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (bit << P::INDEX)) }),
        3 => peripherals
            .gpiod
            .pddr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (bit << P::INDEX)) }),
        4 => peripherals
            .gpioe
            .pddr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (bit << P::INDEX)) }),

        _ => unreachable!(),
    };
}

fn set_data_output<P: Pin>(high: bool) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    if high {
        match P::PORT {
            0 => peripherals
                .gpioa
                .psor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            1 => peripherals
                .gpiob
                .psor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            2 => peripherals
                .gpioc
                .psor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            3 => peripherals
                .gpiod
                .psor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            4 => peripherals
                .gpioe
                .psor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),

            _ => unreachable!(),
        };
    } else {
        match P::PORT {
            0 => peripherals
                .gpioa
                .pcor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            1 => peripherals
                .gpiob
                .pcor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            2 => peripherals
                .gpioc
                .pcor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            3 => peripherals
                .gpiod
                .pcor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),
            4 => peripherals
                .gpioe
                .pcor()
                .write(|w| unsafe { w.bits(1 << P::INDEX) }),

            _ => unreachable!(),
        };
    }
}

fn data_input<P: Pin>() -> bool {
    let peripherals = unsafe { pac::Peripherals::steal() };
    let word: u32 = match P::PORT {
        0 => peripherals.gpioa.pdir().read().pdi().bits(),
        1 => peripherals.gpiob.pdir().read().pdi().bits(),
        2 => peripherals.gpioc.pdir().read().pdi().bits(),
        3 => peripherals.gpiod.pdir().read().pdi().bits(),
        4 => peripherals.gpioe.pdir().read().pdi().bits(),

        _ => unreachable!(),
    };

    (word >> P::INDEX) & 0b0000_0000_0000_0001 != 0
}
