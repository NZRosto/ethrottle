//! MC33HB2001 SPI driver.

#![no_std]

pub use bitfields::{
    BridgeMode, Configuration, ControlMode, CurrentLimit, Input, SlewRate, Status, StatusMask,
};

mod bitfields;

const IDENT: u16 = 0b0000_0000_0000_0010;
const DEFAULT_CONFIGURATION: u16 = 0b0000_1101_1001_1000;

/// Possible setup errors.
#[derive(Debug, defmt::Format)]
pub enum Setup<ESPI, EEN, EDIS> {
    /// Error with the SPI bus.
    Spi(ESPI),
    /// Error enabling device.
    Enable(EEN),
    /// Error setting disable low.
    Disable(EDIS),
    /// Received ident was incorrect.
    IncorrectIdent(u16),
    /// Failed to correctly read-back a modified register.
    CouldNotModifyRegisters,
}

/// MC33HB2001 SPI driver.
pub struct Driver<SPI, EN, DIS, DEL> {
    spi: SPI,
    enable: EN,
    disable: DIS,
    delay: DEL,
}

impl<SPI, EN, DIS, DEL, ESPI, EEN, EDIS> Driver<SPI, EN, DIS, DEL>
where
    SPI: embedded_hal_async::spi::SpiDevice<Error = ESPI>,
    EN: embedded_hal::digital::OutputPin<Error = EEN>,
    DIS: embedded_hal::digital::OutputPin<Error = EDIS>,
    DEL: embedded_hal_async::delay::DelayNs,
{
    /// Creates a new MC33HB2001 Driver using the provided SPI device, enable
    /// and disable pins, and delay implementation.
    ///
    /// # Errors
    /// Returns an error if the driver could not be set up. Setup includes
    /// checks to ensure correct register operation.
    pub async fn new(
        spi: SPI,
        enable: EN,
        disable: DIS,
        delay: DEL,
    ) -> Result<Self, Setup<ESPI, EEN, EDIS>> {
        let mut this = Self {
            spi,
            enable,
            disable,
            delay,
        };

        this.setup().await?;

        Ok(this)
    }

    async fn setup(&mut self) -> Result<(), Setup<ESPI, EEN, EDIS>> {
        self.disable.set_low().map_err(Setup::Disable)?;
        self.enable.set_high().map_err(Setup::Enable)?;

        self.delay.delay_ms(1).await;

        // Read some basic values
        {
            let ident = self
                .read(Register::Identification)
                .await
                .map_err(Setup::Spi)?;

            if ident != IDENT {
                return Err(Setup::IncorrectIdent(ident));
            }

            let _mask = self
                .read(Register::FaultStatusMask)
                .await
                .map_err(Setup::Spi)?;
            let _control = self
                .read(Register::ConfigAndControl)
                .await
                .map_err(Setup::Spi)?;
            let _status = self.read(Register::Status).await.map_err(Setup::Spi)?;
        }

        // Make sure the registers can be written to
        {
            let test_val = 0b0000_1101_1101_1000;

            self.write(Register::ConfigAndControl, test_val)
                .await
                .map_err(Setup::Spi)?;

            let control = self
                .read(Register::ConfigAndControl)
                .await
                .map_err(Setup::Spi)?;

            if control != test_val {
                return Err(Setup::CouldNotModifyRegisters);
            }

            self.write(Register::ConfigAndControl, DEFAULT_CONFIGURATION)
                .await
                .map_err(Setup::Spi)?;
        }

        Ok(())
    }

    /// Get the configuration and control register content.
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn configuration(&mut self) -> Result<Configuration, ESPI> {
        Ok(self.read(Register::ConfigAndControl).await?.into())
    }

    /// Set the configuration and control register content.
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn set_configuration(&mut self, c: Configuration) -> Result<(), ESPI> {
        self.write(Register::ConfigAndControl, c.into_bits()).await
    }

    /// Get the status register content.
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn status(&mut self) -> Result<Status, ESPI> {
        Ok(self.read(Register::Status).await?.into())
    }

    /// Clears the provided flags in the status register.
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn clear_status(&mut self, c: Status) -> Result<(), ESPI> {
        self.write(Register::Status, c.into_bits()).await
    }

    /// Get the status mask register content.
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn status_mask(&mut self) -> Result<StatusMask, ESPI> {
        Ok(self.read(Register::FaultStatusMask).await?.into())
    }

    /// Set the status mask register content
    ///
    /// # Errors
    /// Propagates errors from the SPI bus.
    pub async fn set_status_mask(&mut self, c: StatusMask) -> Result<(), ESPI> {
        self.write(Register::FaultStatusMask, c.into_bits()).await
    }
}

#[derive(Clone, Copy)]
#[repr(u16)]
enum Register {
    Identification = 0b0000_0000_0000_0000,
    Status = 0b0010_0000_0000_0000,
    FaultStatusMask = 0b0100_0000_0000_0000,
    ConfigAndControl = 0b0110_0000_0000_0000,
}

impl<SPI, EN, DIS, DEL, ESPI> Driver<SPI, EN, DIS, DEL>
where
    SPI: embedded_hal_async::spi::SpiDevice<Error = ESPI>,
{
    async fn write(&mut self, register: Register, mut data: u16) -> Result<(), ESPI> {
        data &= 0b0001_1111_1111_1111;
        data |= 0b1000_0000_0000_0000; // Write operation
        data |= register as u16;
        // Workaround because responses are delayed by one CS cycle.
        let mut buf = [0_u8; 2];
        self.spi.transfer(&mut buf, &data.to_be_bytes()).await?;
        Ok(())
    }

    async fn read(&mut self, register: Register) -> Result<u16, ESPI> {
        let data = register as u16;
        let mut buf = [0_u8; 2];
        // Workaround because responses are delayed by one CS cycle.
        self.spi.transfer(&mut buf, &data.to_be_bytes()).await?;
        self.spi.transfer(&mut buf, &[0_u8; 2]).await?;
        Ok(u16::from_be_bytes(buf) & 0b0001_1111_1111_1111)
    }
}
