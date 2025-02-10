pub mod pwm;

mod sealed {
    pub trait Sealed {}
}

pub trait Timer: sealed::Sealed {
    const INDEX: u32;
}

#[allow(clippy::module_name_repetitions)]
pub struct Tpm0 {
    _private: (),
}

impl Tpm0 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl sealed::Sealed for Tpm0 {}

impl Timer for Tpm0 {
    const INDEX: u32 = 0;
}

#[allow(clippy::module_name_repetitions)]
pub struct Tpm1 {
    _private: (),
}

impl Tpm1 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl sealed::Sealed for Tpm1 {}

impl Timer for Tpm1 {
    const INDEX: u32 = 1;
}

#[allow(clippy::module_name_repetitions)]
pub struct Tpm2 {
    _private: (),
}

impl Tpm2 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl sealed::Sealed for Tpm2 {}

impl Timer for Tpm2 {
    const INDEX: u32 = 2;
}

pub trait TimerPin<T, const CHANNEL: u32> {
    const ALT: crate::mux::Alternate;
}

macro_rules! t_pin {
    ($pin:ident, $tpm:ident, $ch:literal, $alt:ident) => {
        impl TimerPin<$tpm, $ch> for crate::pins::$pin {
            const ALT: crate::mux::Alternate = crate::mux::Alternate::$alt;
        }
    };
}

t_pin!(PTE20, Tpm1, 0, Alt3);
t_pin!(PTE21, Tpm1, 1, Alt3);
t_pin!(PTE22, Tpm2, 0, Alt3);
t_pin!(PTE23, Tpm2, 1, Alt3);
t_pin!(PTE29, Tpm0, 2, Alt3);
t_pin!(PTE30, Tpm0, 3, Alt3);
t_pin!(PTE31, Tpm0, 4, Alt3);
t_pin!(PTE24, Tpm0, 0, Alt3);
t_pin!(PTE25, Tpm0, 1, Alt3);
t_pin!(PTA0, Tpm0, 5, Alt3);
t_pin!(PTA1, Tpm2, 0, Alt3);
t_pin!(PTA2, Tpm2, 1, Alt3);
t_pin!(PTA3, Tpm0, 0, Alt3);
t_pin!(PTA4, Tpm0, 1, Alt3);
t_pin!(PTA5, Tpm0, 2, Alt3);
t_pin!(PTA12, Tpm1, 0, Alt3);
t_pin!(PTA13, Tpm1, 1, Alt3);
t_pin!(PTB0, Tpm1, 0, Alt3);
t_pin!(PTB1, Tpm1, 1, Alt3);
t_pin!(PTB2, Tpm2, 0, Alt3);
t_pin!(PTB3, Tpm2, 1, Alt3);
t_pin!(PTB18, Tpm2, 0, Alt3);
t_pin!(PTB19, Tpm2, 1, Alt3);
t_pin!(PTC1, Tpm0, 0, Alt4);
t_pin!(PTC2, Tpm0, 1, Alt4);
t_pin!(PTC3, Tpm0, 2, Alt4);
t_pin!(PTC4, Tpm0, 3, Alt4);
t_pin!(PTC8, Tpm0, 4, Alt3);
t_pin!(PTC9, Tpm0, 5, Alt3);
t_pin!(PTD0, Tpm0, 0, Alt4);
t_pin!(PTD1, Tpm0, 1, Alt4);
t_pin!(PTD2, Tpm0, 2, Alt4);
t_pin!(PTD3, Tpm0, 3, Alt4);
t_pin!(PTD4, Tpm0, 4, Alt4);
t_pin!(PTD5, Tpm0, 5, Alt4);

fn ensure_clock_active<T: Timer>() {
    let peripherals = unsafe { pac::Peripherals::steal() };

    peripherals.sim.scgc6().modify(|_, w| match T::INDEX {
        0 => w.tpm0()._1(),
        1 => w.tpm1()._1(),
        2 => w.tpm2()._1(),

        _ => unreachable!(),
    });
}

fn enable_timer<T: Timer>(enable: bool) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    match T::INDEX {
        0 => peripherals.tpm0.sc().write(|w| {
            w.cmod().variant(if enable {
                pac::tpm0::sc::Cmod::_01
            } else {
                pac::tpm0::sc::Cmod::_00
            })
        }),
        1 => peripherals.tpm1.sc().write(|w| {
            w.cmod().variant(if enable {
                pac::tpm1::sc::Cmod::_01
            } else {
                pac::tpm1::sc::Cmod::_00
            })
        }),
        2 => peripherals.tpm2.sc().write(|w| {
            w.cmod().variant(if enable {
                pac::tpm2::sc::Cmod::_01
            } else {
                pac::tpm2::sc::Cmod::_00
            })
        }),

        _ => unreachable!(),
    };
}

fn set_timer_mod_value<T: Timer>(val: u16) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    unsafe {
        match T::INDEX {
            0 => peripherals.tpm0.mod_().write(|w| w.mod_().bits(val)),
            1 => peripherals.tpm1.mod_().write(|w| w.mod_().bits(val)),
            2 => peripherals.tpm2.mod_().write(|w| w.mod_().bits(val)),

            _ => unreachable!(),
        };
    }
}

fn disable_channel<T: Timer, const CHANNEL: u32>() {
    let peripherals = unsafe { pac::Peripherals::steal() };

    match (T::INDEX, CHANNEL) {
        (0, 0) => peripherals.tpm0.c0sc().write(|w| unsafe { w.bits(0) }),
        (0, 1) => peripherals.tpm0.c1sc().write(|w| unsafe { w.bits(0) }),
        (0, 2) => peripherals.tpm0.c2sc().write(|w| unsafe { w.bits(0) }),
        (0, 3) => peripherals.tpm0.c3sc().write(|w| unsafe { w.bits(0) }),
        (0, 4) => peripherals.tpm0.c4sc().write(|w| unsafe { w.bits(0) }),
        (0, 5) => peripherals.tpm0.c5sc().write(|w| unsafe { w.bits(0) }),

        (1, 0) => peripherals.tpm1.c0sc().write(|w| unsafe { w.bits(0) }),
        (1, 1) => peripherals.tpm1.c1sc().write(|w| unsafe { w.bits(0) }),

        (2, 0) => peripherals.tpm2.c0sc().write(|w| unsafe { w.bits(0) }),
        (2, 1) => peripherals.tpm2.c1sc().write(|w| unsafe { w.bits(0) }),

        _ => unreachable!(),
    };
}

#[allow(clippy::too_many_lines)]
fn configure_for_pwm<T: Timer, const CHANNEL: u32>() {
    let peripherals = unsafe { pac::Peripherals::steal() };

    match (T::INDEX, CHANNEL) {
        (0, 0) => peripherals.tpm0.c0sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (0, 1) => peripherals.tpm0.c1sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (0, 2) => peripherals.tpm0.c2sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (0, 3) => peripherals.tpm0.c3sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (0, 4) => peripherals.tpm0.c4sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (0, 5) => peripherals.tpm0.c5sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (1, 0) => peripherals.tpm1.c0sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (1, 1) => peripherals.tpm1.c1sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (2, 0) => peripherals.tpm2.c0sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        (2, 1) => peripherals.tpm2.c1sc().write(|w| {
            w.msb()
                .set_bit()
                .msa()
                .clear_bit()
                .elsb()
                .set_bit()
                .elsa()
                .clear_bit()
        }),

        _ => unreachable!(),
    };
}

fn set_channel_value<T: Timer, const CHANNEL: u32>(val: u16) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    unsafe {
        match (T::INDEX, CHANNEL) {
            (0, 0) => peripherals.tpm0.c0v().write(|w| w.val().bits(val)),
            (0, 1) => peripherals.tpm0.c1v().write(|w| w.val().bits(val)),
            (0, 2) => peripherals.tpm0.c2v().write(|w| w.val().bits(val)),
            (0, 3) => peripherals.tpm0.c3v().write(|w| w.val().bits(val)),
            (0, 4) => peripherals.tpm0.c4v().write(|w| w.val().bits(val)),
            (0, 5) => peripherals.tpm0.c5v().write(|w| w.val().bits(val)),

            (1, 0) => peripherals.tpm1.c0v().write(|w| w.val().bits(val)),
            (1, 1) => peripherals.tpm1.c1v().write(|w| w.val().bits(val)),

            (2, 0) => peripherals.tpm2.c0v().write(|w| w.val().bits(val)),
            (2, 1) => peripherals.tpm2.c1v().write(|w| w.val().bits(val)),

            _ => unreachable!(),
        };
    }
}
