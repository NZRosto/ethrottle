mod sealed {
    pub trait Sealed {}
}

pub trait Pin: sealed::Sealed {
    const INDEX: u32;
    const PORT: u32;
}

macro_rules! pins {
    (
        $(($name:ident, $port:literal, $index:literal)),*
    ) => {
        $(
            pub struct $name {
                _private: (),
            }

            impl sealed::Sealed for $name {}

            impl Pin for $name {
                const INDEX: u32 = $index;
                const PORT: u32 = $port;
            }
        )*

        #[allow(non_snake_case)]
        pub struct Pins {
            $(
                pub $name: $name,
            )*
        }

        impl Pins {
            pub(crate) fn new() -> Self {
                Self {
                    $(
                        $name: $name { _private: () },
                    )*
                }
            }
        }
    };
}

pins!(
    (PTA0, 0, 0),
    (PTA1, 0, 1),
    (PTA2, 0, 2),
    (PTA3, 0, 3),
    (PTA4, 0, 4),
    (PTA5, 0, 5),
    (PTA6, 0, 6),
    (PTA7, 0, 7),
    (PTA8, 0, 8),
    (PTA9, 0, 9),
    (PTA10, 0, 10),
    (PTA11, 0, 11),
    (PTA12, 0, 12),
    (PTA13, 0, 13),
    (PTA14, 0, 14),
    (PTA15, 0, 15),
    (PTA16, 0, 16),
    (PTA17, 0, 17),
    (PTA18, 0, 18),
    (PTA19, 0, 19),
    (PTA20, 0, 20),
    (PTA21, 0, 21),
    (PTA22, 0, 22),
    (PTA23, 0, 23),
    (PTA24, 0, 24),
    (PTA25, 0, 25),
    (PTA26, 0, 26),
    (PTA27, 0, 27),
    (PTA28, 0, 28),
    (PTA29, 0, 29),
    (PTA30, 0, 30),
    (PTA31, 0, 31),
    (PTB0, 1, 0),
    (PTB1, 1, 1),
    (PTB2, 1, 2),
    (PTB3, 1, 3),
    (PTB4, 1, 4),
    (PTB5, 1, 5),
    (PTB6, 1, 6),
    (PTB7, 1, 7),
    (PTB8, 1, 8),
    (PTB9, 1, 9),
    (PTB10, 1, 10),
    (PTB11, 1, 11),
    (PTB12, 1, 12),
    (PTB13, 1, 13),
    (PTB14, 1, 14),
    (PTB15, 1, 15),
    (PTB16, 1, 16),
    (PTB17, 1, 17),
    (PTB18, 1, 18),
    (PTB19, 1, 19),
    (PTB20, 1, 20),
    (PTB21, 1, 21),
    (PTB22, 1, 22),
    (PTB23, 1, 23),
    (PTB24, 1, 24),
    (PTB25, 1, 25),
    (PTB26, 1, 26),
    (PTB27, 1, 27),
    (PTB28, 1, 28),
    (PTB29, 1, 29),
    (PTB30, 1, 30),
    (PTB31, 1, 31),
    (PTC0, 2, 0),
    (PTC1, 2, 1),
    (PTC2, 2, 2),
    (PTC3, 2, 3),
    (PTC4, 2, 4),
    (PTC5, 2, 5),
    (PTC6, 2, 6),
    (PTC7, 2, 7),
    (PTC8, 2, 8),
    (PTC9, 2, 9),
    (PTC10, 2, 10),
    (PTC11, 2, 11),
    (PTC12, 2, 12),
    (PTC13, 2, 13),
    (PTC14, 2, 14),
    (PTC15, 2, 15),
    (PTC16, 2, 16),
    (PTC17, 2, 17),
    (PTC18, 2, 18),
    (PTC19, 2, 19),
    (PTC20, 2, 20),
    (PTC21, 2, 21),
    (PTC22, 2, 22),
    (PTC23, 2, 23),
    (PTC24, 2, 24),
    (PTC25, 2, 25),
    (PTC26, 2, 26),
    (PTC27, 2, 27),
    (PTC28, 2, 28),
    (PTC29, 2, 29),
    (PTC30, 2, 30),
    (PTC31, 2, 31),
    (PTD0, 3, 0),
    (PTD1, 3, 1),
    (PTD2, 3, 2),
    (PTD3, 3, 3),
    (PTD4, 3, 4),
    (PTD5, 3, 5),
    (PTD6, 3, 6),
    (PTD7, 3, 7),
    (PTD8, 3, 8),
    (PTD9, 3, 9),
    (PTD10, 3, 10),
    (PTD11, 3, 11),
    (PTD12, 3, 12),
    (PTD13, 3, 13),
    (PTD14, 3, 14),
    (PTD15, 3, 15),
    (PTD16, 3, 16),
    (PTD17, 3, 17),
    (PTD18, 3, 18),
    (PTD19, 3, 19),
    (PTD20, 3, 20),
    (PTD21, 3, 21),
    (PTD22, 3, 22),
    (PTD23, 3, 23),
    (PTD24, 3, 24),
    (PTD25, 3, 25),
    (PTD26, 3, 26),
    (PTD27, 3, 27),
    (PTD28, 3, 28),
    (PTD29, 3, 29),
    (PTD30, 3, 30),
    (PTD31, 3, 31),
    (PTE0, 4, 0),
    (PTE1, 4, 1),
    (PTE2, 4, 2),
    (PTE3, 4, 3),
    (PTE4, 4, 4),
    (PTE5, 4, 5),
    (PTE6, 4, 6),
    (PTE7, 4, 7),
    (PTE8, 4, 8),
    (PTE9, 4, 9),
    (PTE10, 4, 10),
    (PTE11, 4, 11),
    (PTE12, 4, 12),
    (PTE13, 4, 13),
    (PTE14, 4, 14),
    (PTE15, 4, 15),
    (PTE16, 4, 16),
    (PTE17, 4, 17),
    (PTE18, 4, 18),
    (PTE19, 4, 19),
    (PTE20, 4, 20),
    (PTE21, 4, 21),
    (PTE22, 4, 22),
    (PTE23, 4, 23),
    (PTE24, 4, 24),
    (PTE25, 4, 25),
    (PTE26, 4, 26),
    (PTE27, 4, 27),
    (PTE28, 4, 28),
    (PTE29, 4, 29),
    (PTE30, 4, 30),
    (PTE31, 4, 31)
);

pub(crate) fn enable_port_clock<P: Pin>() {
    unsafe { pac::Sim::steal() }
        .scgc5()
        .modify(|_, w| match P::PORT {
            0 => w.porta()._1(),
            1 => w.portb()._1(),
            2 => w.portc()._1(),
            3 => w.portd()._1(),
            4 => w.porte()._1(),

            _ => unreachable!(),
        });
}
