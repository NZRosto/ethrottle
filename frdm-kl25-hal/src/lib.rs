#![no_std]
#![allow(missing_docs)]

pub mod dma;
pub mod gpio;
mod mux;
pub mod pins;
pub mod spi;
pub mod tpm;

pub use cortex_m_rt::entry;

pub struct Peripherals {
    pub pins: pins::Pins,
    pub tpm0: tpm::Tpm0,
    pub tpm1: tpm::Tpm1,
    pub tpm2: tpm::Tpm2,
    pub dma0: dma::Dma0,
    pub dma1: dma::Dma1,
    pub dma2: dma::Dma2,
    pub dma3: dma::Dma3,
    pub spi0: spi::Spi0,
    pub spi1: spi::Spi1,
}

impl Peripherals {
    /// Steal the set of peripherals that represent this board.
    ///
    /// # Safety
    /// Must only be called once, multiple sets of peripherals may cause
    /// undefined and unexpected behaviour.
    #[must_use]
    pub unsafe fn steal() -> Self {
        let peripherals = pac::Peripherals::steal();

        peripherals.sim.copc().write(|w| w.copt()._00()); // Disable watchdog

        // Setup clock for 48Mhz operation.
        {
            peripherals.mcg.c2().write(|w| w.bits(0x14_u8));
            peripherals.mcg.c1().write(|w| w.bits(0x90_u8));

            while peripherals.mcg.s().read().oscinit0().bit_is_clear() {}
            while peripherals.mcg.s().read().irefst().bit_is_set() {}
            while !peripherals.mcg.s().read().clkst().is_10() {}

            peripherals.mcg.c5().write(|w| w.prdiv0()._3());
            peripherals.mcg.c6().write(|w| w.bits(0x40_u8));

            while peripherals.mcg.s().read().pllst().bit_is_clear() {}
            while peripherals.mcg.s().read().lock0().bit_is_clear() {}

            peripherals.mcg.c1().write(|w| w.bits(0x10_u8));

            while !peripherals.mcg.s().read().clkst().is_11() {}
        }

        peripherals
            .sim
            .sopt2()
            .write(|w| w.tpmsrc()._01().pllfllsel()._1()); // Select PLL for TPM clock

        Self {
            pins: pins::Pins::new(),
            tpm0: tpm::Tpm0::new(),
            tpm1: tpm::Tpm1::new(),
            tpm2: tpm::Tpm2::new(),
            dma0: dma::Dma0::new(),
            dma1: dma::Dma1::new(),
            dma2: dma::Dma2::new(),
            dma3: dma::Dma3::new(),
            spi0: spi::Spi0::new(),
            spi1: spi::Spi1::new(),
        }
    }
}
