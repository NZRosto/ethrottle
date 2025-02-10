#![allow(clippy::module_name_repetitions)]

pub trait Dma {
    const INDEX: u32;
}

pub struct Dma0 {
    _private: (),
}

impl Dma0 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Dma for Dma0 {
    const INDEX: u32 = 0;
}

pub struct Dma1 {
    _private: (),
}

impl Dma1 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Dma for Dma1 {
    const INDEX: u32 = 1;
}

pub struct Dma2 {
    _private: (),
}

impl Dma2 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Dma for Dma2 {
    const INDEX: u32 = 2;
}

pub struct Dma3 {
    _private: (),
}

impl Dma3 {
    pub(crate) fn new() -> Self {
        Self { _private: () }
    }
}

impl Dma for Dma3 {
    const INDEX: u32 = 3;
}

#[allow(clippy::too_many_lines)]
#[allow(clippy::fn_params_excessive_bools)]
pub(crate) unsafe fn setup_dma_transfer<D: Dma>(
    source_addr: *const u8,
    dest_addr: *mut u8,
    num_bytes: u16,
    inc_source: bool,
    inc_dest: bool,
    start: bool,
    auto_inc: bool,
) {
    let peripherals = unsafe { pac::Peripherals::steal() };

    let source_addr = source_addr as u32;
    let dest_addr = dest_addr as u32;

    // Reset DMA
    match D::INDEX {
        0 => peripherals.dma.dsr_bcr0().write(|w| w.done()._1()),
        1 => peripherals.dma.dsr_bcr1().write(|w| w.done()._1()),
        2 => peripherals.dma.dsr_bcr2().write(|w| w.done()._1()),
        3 => peripherals.dma.dsr_bcr3().write(|w| w.done()._1()),

        _ => unreachable!(),
    };

    // Set source address
    match D::INDEX {
        0 => peripherals.dma.sar0().write(|w| w.sar().bits(source_addr)),
        1 => peripherals.dma.sar1().write(|w| w.sar().bits(source_addr)),
        2 => peripherals.dma.sar2().write(|w| w.sar().bits(source_addr)),
        3 => peripherals.dma.sar3().write(|w| w.sar().bits(source_addr)),

        _ => unreachable!(),
    };

    // Set destination address
    match D::INDEX {
        0 => peripherals.dma.dar0().write(|w| w.dar().bits(dest_addr)),
        1 => peripherals.dma.dar1().write(|w| w.dar().bits(dest_addr)),
        2 => peripherals.dma.dar2().write(|w| w.dar().bits(dest_addr)),
        3 => peripherals.dma.dar3().write(|w| w.dar().bits(dest_addr)),

        _ => unreachable!(),
    };

    match D::INDEX {
        0 => peripherals
            .dma
            .dsr_bcr0()
            .write(|w| w.bcr().bits(num_bytes.into())),
        1 => peripherals
            .dma
            .dsr_bcr1()
            .write(|w| w.bcr().bits(num_bytes.into())),
        2 => peripherals
            .dma
            .dsr_bcr2()
            .write(|w| w.bcr().bits(num_bytes.into())),
        3 => peripherals
            .dma
            .dsr_bcr3()
            .write(|w| w.bcr().bits(num_bytes.into())),

        _ => unreachable!(),
    };

    match D::INDEX {
        0 => peripherals.dma.dcr0().write(|w| {
            w.sinc()
                .variant(if inc_source {
                    pac::dma::dcr0::Sinc::_1
                } else {
                    pac::dma::dcr0::Sinc::_0
                })
                .ssize()
                ._01()
                .dinc()
                .variant(if inc_dest {
                    pac::dma::dcr0::Dinc::_1
                } else {
                    pac::dma::dcr0::Dinc::_0
                })
                .dsize()
                ._01()
                .erq()
                ._1()
                .start()
                .bit(start)
                .d_req()
                .bit(auto_inc)
                .cs()
                .bit(auto_inc)
        }),

        1 => peripherals.dma.dcr1().write(|w| {
            w.sinc()
                .variant(if inc_source {
                    pac::dma::dcr1::Sinc::_1
                } else {
                    pac::dma::dcr1::Sinc::_0
                })
                .ssize()
                ._01()
                .dinc()
                .variant(if inc_dest {
                    pac::dma::dcr1::Dinc::_1
                } else {
                    pac::dma::dcr1::Dinc::_0
                })
                .dsize()
                ._01()
                .erq()
                ._1()
                .start()
                .bit(start)
                .d_req()
                .bit(auto_inc)
                .cs()
                .bit(auto_inc)
        }),

        2 => peripherals.dma.dcr2().write(|w| {
            w.sinc()
                .variant(if inc_source {
                    pac::dma::dcr2::Sinc::_1
                } else {
                    pac::dma::dcr2::Sinc::_0
                })
                .ssize()
                ._01()
                .dinc()
                .variant(if inc_dest {
                    pac::dma::dcr2::Dinc::_1
                } else {
                    pac::dma::dcr2::Dinc::_0
                })
                .dsize()
                ._01()
                .erq()
                ._1()
                .start()
                .bit(start)
                .d_req()
                .bit(auto_inc)
                .cs()
                .bit(auto_inc)
        }),

        3 => peripherals.dma.dcr3().write(|w| {
            w.sinc()
                .variant(if inc_source {
                    pac::dma::dcr3::Sinc::_1
                } else {
                    pac::dma::dcr3::Sinc::_0
                })
                .ssize()
                ._01()
                .dinc()
                .variant(if inc_dest {
                    pac::dma::dcr3::Dinc::_1
                } else {
                    pac::dma::dcr3::Dinc::_0
                })
                .dsize()
                ._01()
                .erq()
                ._1()
                .start()
                .bit(start)
                .d_req()
                .bit(auto_inc)
                .cs()
                .bit(auto_inc)
        }),

        _ => unreachable!(),
    };
}

pub(crate) async unsafe fn wait_dma_transfer<D: Dma>() {
    let peripherals = unsafe { pac::Peripherals::steal() };

    core::future::poll_fn(|_cx| {
        let done = match D::INDEX {
            0 => peripherals.dma.dsr_bcr0().read().done().is_1(),
            1 => peripherals.dma.dsr_bcr1().read().done().is_1(),
            2 => peripherals.dma.dsr_bcr2().read().done().is_1(),
            3 => peripherals.dma.dsr_bcr3().read().done().is_1(),

            _ => unreachable!(),
        };

        if done {
            match D::INDEX {
                0 => peripherals.dma.dsr_bcr0().write(|w| w.done()._1()),
                1 => peripherals.dma.dsr_bcr1().write(|w| w.done()._1()),
                2 => peripherals.dma.dsr_bcr2().write(|w| w.done()._1()),
                3 => peripherals.dma.dsr_bcr3().write(|w| w.done()._1()),

                _ => unreachable!(),
            };

            core::task::Poll::Ready(())
        } else {
            core::task::Poll::Pending
        }
    })
    .await;
}
