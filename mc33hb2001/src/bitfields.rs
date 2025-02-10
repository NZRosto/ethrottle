/// Current limit values. Units of Amperes.
#[repr(u8)]
#[derive(Debug, defmt::Format)]
pub enum CurrentLimit {
    /// Limit to 5.4A.
    Lim5_4 = 0b00,
    /// Limit to 7.0A.
    Lim7_0 = 0b01,
    /// Limit to 8.8A.
    Lim8_8 = 0b10,
    /// Limit to 10.7A.
    Lim10_7 = 0b11,
}

impl CurrentLimit {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0b00 => CurrentLimit::Lim5_4,
            0b01 => CurrentLimit::Lim7_0,
            0b10 => CurrentLimit::Lim8_8,
            0b11 => CurrentLimit::Lim10_7,
            _ => unreachable!(),
        }
    }
}

/// Slew rate values. Units of volts per microsecond.
#[repr(u8)]
#[derive(Debug, defmt::Format)]
pub enum SlewRate {
    /// Bypass slew rate control.
    Bypass = 0b000,
    /// 16.0 V/μs.
    Sr16_0 = 0b001,
    /// 8.0 V/μs.
    Sr8_0 = 0b010,
    /// 4.0 V/μs.
    Sr4_0 = 0b011,
    /// 2.0 V/μs.
    Sr2_0 = 0b100,
    /// 1.0 V/μs.
    Sr1_0 = 0b101,
    /// 0.5 V/μs.
    Sr0_5 = 0b110,
    /// 0.25 V/μs.
    Sr0_25 = 0b111,
}

impl SlewRate {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0b000 => SlewRate::Bypass,
            0b001 => SlewRate::Sr16_0,
            0b010 => SlewRate::Sr8_0,
            0b011 => SlewRate::Sr4_0,
            0b100 => SlewRate::Sr2_0,
            0b101 => SlewRate::Sr1_0,
            0b110 => SlewRate::Sr0_5,
            0b111 => SlewRate::Sr0_25,
            _ => unreachable!(),
        }
    }
}

/// Input bridge mode.
#[repr(u8)]
#[derive(Debug, defmt::Format)]
pub enum BridgeMode {
    /// Half-bridge control mode.
    HalfBridge = 0b0,
    /// H-bridge control mode.
    HBridge = 0b1,
}

impl BridgeMode {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0b0 => BridgeMode::HalfBridge,
            0b1 => BridgeMode::HBridge,
            _ => unreachable!(),
        }
    }
}

/// Input control mode.
#[repr(u8)]
#[derive(Debug, defmt::Format)]
pub enum ControlMode {
    /// Parallel pin control, SPI virtual inputs disabled.
    Parallel = 0b0,
    /// SPI virtual control, Parallel pins disabled.
    Spi = 0b1,
}

impl ControlMode {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0b0 => ControlMode::Parallel,
            0b1 => ControlMode::Spi,
            _ => unreachable!(),
        }
    }
}

/// The logic value of a driver input
#[repr(u8)]
#[derive(Debug, defmt::Format)]
pub enum Input {
    /// Logic low.
    Low = 0b0,
    /// Logic high.
    High = 0b1,
}

impl Input {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0b0 => Input::Low,
            0b1 => Input::High,
            _ => unreachable!(),
        }
    }
}

/// Configuration and control register.
#[bitfield_struct::bitfield(u16, defmt = true, order = Msb)]
pub struct Configuration {
    #[bits(3)]
    __: u8,

    /// Check for open load (in Full Bridge Standby mode). When this becomes
    /// true and the driver is in Standby, a test will be immediately executed.
    /// In addition the open load test is enabled on transition from Standby to
    /// Normal mode.
    #[bits(1, default = false)]
    pub check_for_open_load: bool,

    /// Enable change of current limit frequency when in over-temperature
    /// warning state.
    #[bits(1, default = true)]
    pub enable_thermal_management: bool,

    /// Enable active current limit when specified threshold has been exceeded.
    /// When this is disabled the overcurrent flag can still be set, but outputs
    /// are not changed.
    #[bits(1, default = true)]
    pub enable_active_current_limit: bool,

    /// Active current limit value.
    #[bits(2, default = CurrentLimit::Lim7_0)]
    pub current_limit: CurrentLimit,

    /// Slew rate value.
    #[bits(3, default = SlewRate::Sr2_0)]
    pub slew_rate: SlewRate,

    /// Enable output control when ENBL pin is high and DIS pin is low.
    #[bits(1, default = true)]
    pub enable: bool,

    /// Input bridge mode.
    #[bits(1, default = BridgeMode::HBridge)]
    pub bridge_mode: BridgeMode,

    /// Input control mode.
    #[bits(1, default = ControlMode::Parallel)]
    pub control_mode: ControlMode,

    /// Virtual input 2
    #[bits(1, default = Input::Low)]
    pub virtual_input_2: Input,

    /// Virtual input 1
    #[bits(1, default = Input::Low)]
    pub virtual_input_1: Input,
}

/// Status register.
#[bitfield_struct::bitfield(u16, defmt = true, order = Msb)]
pub struct Status {
    #[bits(4)]
    __: u8,

    /// Spi framing error has occurred.
    #[bits(1)]
    pub spi_framing_error: bool,

    /// Charge pump over-voltage has occurred.
    #[bits(1)]
    pub charge_pump_overvoltage: bool,

    /// Main power under-voltage has occurred.
    #[bits(1)]
    pub vpwr_undervoltage: bool,

    /// Main power over-voltage has occurred.
    #[bits(1)]
    pub vpwr_overvoltage: bool,

    /// Short-circuit to power output 2 has occurred.
    #[bits(1)]
    pub sc_power_output_2: bool,

    /// Short-circuit to power output 1 has occurred.
    #[bits(1)]
    pub sc_power_output_1: bool,

    /// Short-circuit to ground output 2 has occurred.
    #[bits(1)]
    pub sc_ground_output_2: bool,

    /// Short-circuit to ground output 1 has occurred.
    #[bits(1)]
    pub sc_ground_output_1: bool,

    /// Output is open-loaded.
    #[bits(1)]
    pub open_load: bool,

    /// Current limit has been activated.
    #[bits(1)]
    pub overcurrent: bool,

    /// Thermal warning has occurred.
    #[bits(1)]
    pub thermal_warning: bool,

    /// Shutdown has occurred due to over-temperature.
    #[bits(1)]
    pub overtemperature_shutdown: bool,
}

/// Status mask register.
#[bitfield_struct::bitfield(u16, defmt = true, order = Msb)]
pub struct StatusMask {
    #[bits(3)]
    __: u8,

    /// Disable overvoltage protection. Overvoltage status flag becomes a
    /// warning only.
    #[bits(1)]
    pub disable_overvoltage: bool,

    /// Spi framing error has occurred.
    #[bits(1)]
    pub spi_framing_error: bool,

    /// Charge pump over-voltage has occurred.
    #[bits(1)]
    pub charge_pump_overvoltage: bool,

    /// Main power under-voltage has occurred.
    #[bits(1)]
    pub vpwr_undervoltage: bool,

    /// Main power over-voltage has occurred.
    #[bits(1)]
    pub vpwr_overvoltage: bool,

    /// Short-circuit to power output 2 has occurred.
    #[bits(1)]
    pub sc_power_output_2: bool,

    /// Short-circuit to power output 1 has occurred.
    #[bits(1)]
    pub sc_power_output_1: bool,

    /// Short-circuit to ground output 2 has occurred.
    #[bits(1)]
    pub sc_ground_output_2: bool,

    /// Short-circuit to ground output 1 has occurred.
    #[bits(1)]
    pub sc_ground_output_1: bool,

    /// Output is open-loaded.
    #[bits(1)]
    pub open_load: bool,

    /// Current limit has been activated.
    #[bits(1)]
    pub overcurrent: bool,

    /// Thermal warning has occurred.
    #[bits(1)]
    pub thermal_warning: bool,

    /// Shutdown has occurred due to over-temperature.
    #[bits(1)]
    pub overtemperature_shutdown: bool,
}
