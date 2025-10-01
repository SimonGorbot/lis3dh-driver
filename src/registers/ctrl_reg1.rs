//! # CTRL_REG1 (20h)
//! ## Fields:
//! - `odr`: Data rate selection.
//! - `lp_en`: Low-power mode enable.
//! - `axis_enable` fields:
//!     - `x_en`: X-axis enable.
//!     - `y_en`: Y-axis enable.
//!     - `z_en`: Z-axis enable.

use crate::registers::{define_state_renderer, Entitled, ReadWriteRegisterAddress};

pub const ADDR: u8 = ReadWriteRegisterAddress::CtrlReg1 as u8;

/// ### `odr`: Data rate selection.
///   - `0b0000`: Power-down mode.
///   - `0b0001`: 1 Hz.
///   - `0b0010`: 10 Hz.
///   - `0b0011`: 25 Hz.
///   - `0b0100`: 50 Hz.
///   - `0b0101`: 100 Hz.
///   - `0b0110`: 200 Hz.
///   - `0b0111`: 400 Hz.
///   - `0b1000`: 1.60 kHz (**Exclusive to low power mode**).
///   - `0b1001`: 1.344 kHz (**Exclusive to normal power mode**)
///   - `0b1001`: 5.376 kHz (**Exclusive to low power mode**)
///
/// *Default value: 0b0000 (PowerDown).*
///
/// ### Entitlements:
///   - Output data rate of [`odr::F1344Hz`] is entitled to [`lp_en::NormalPowerMode`]
///   - Output data rate of [`odr::F1600Hz`] is entitled to [`lp_en::LowPowerMode`]
///   - Output data rate of [`odr::F5376Hz`] is entitled to [`lp_en::LowPowerMode`]
pub mod odr {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 4;
    pub const OFFSET: u8 = 4;
    pub type Default = PowerDown;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        PowerDown = 0b0000,
        F1Hz = 0b0001,
        F10Hz = 0b0010,
        F25Hz = 0b0011,
        F50Hz = 0b0100,
        F100Hz = 0b0101,
        F200Hz = 0b0110,
        F400Hz = 0b0111,
        F1600Hz = 0b1000, // Exclusive to low power mode.
        F1344Hz = 0b1001, // Exclusive to normal power mode.
    }

    // Special case for odr register. The raw value 0b1001 is used to set both 1344Hz in normal power mode, and 5376Hz in low power mode.
    // Entitlements will help keep this clear to the user as they can not set a power mode specific frequency without being in the correct power state.
    // In the future, I might combine fields into a "feature" named OperatingMode that takes the entangled fields odr, lp_en, and hr to avoid this band-aid solution.
    impl Variant {
        pub const F5376HZ: Variant = Variant::F1344Hz;
    }

    macro_rules! impls {
        ($name:ident) => {
            pub struct $name;

            impl State for $name {
                const VARIANT: Variant = Variant::$name;
            }
        };
    }

    impls!(PowerDown); // Equivalent to no data output or 0Hz. PowerDown is used to match the data sheet naming.
    impls!(F1Hz);
    impls!(F10Hz);
    impls!(F25Hz);
    impls!(F50Hz);
    impls!(F100Hz);
    impls!(F200Hz);
    impls!(F400Hz);
    impls!(F1600Hz);
    impls!(F1344Hz);

    // Implementation of State for special 5376Hz odr case.
    pub struct F5376Hz;

    impl State for F5376Hz {
        const VARIANT: Variant = Variant::F5376HZ;
    }
}

// Entitlements of odr bit field.
impl<T: lp_en::State> Entitled<T> for odr::PowerDown {}
impl<T: lp_en::State> Entitled<T> for odr::F1Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F10Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F25Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F50Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F100Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F200Hz {}
impl<T: lp_en::State> Entitled<T> for odr::F400Hz {}
impl Entitled<lp_en::LowPowerMode> for odr::F1600Hz {}
impl Entitled<lp_en::NormalPowerMode> for odr::F1344Hz {}
impl Entitled<lp_en::LowPowerMode> for odr::F5376Hz {}

/// ### `lp_en`: Low-power mode enable.
///   - `0b0`: high-resolution / normal mode.
///   - `0b1`: low-power mode.
///
/// * Default value: 0b0 (normal mode).*
pub mod lp_en {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 3;
    pub type Default = NormalPowerMode;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        NormalPowerMode = 0b0,
        LowPowerMode = 0b1,
    }

    pub struct NormalPowerMode;
    pub struct LowPowerMode;

    impl State for NormalPowerMode {
        const VARIANT: Variant = Variant::NormalPowerMode;
    }

    impl State for LowPowerMode {
        const VARIANT: Variant = Variant::LowPowerMode;
    }
}

/// ### `axis_enable`: Axis Enable Feature made up of the registers below:
/// - `Zen`: Z-axis enable. Default value: 1.
///   - `0b0`: Z-axis disabled.
///   - `0b1`: Z-axis enabled.
/// - `Yen`: Y-axis enable. Default value: 1.
///   - `0b0`: Y-axis disabled.
///   - `0b1`: Y-axis enabled.
/// - `Xen`: X-axis enable. Default value: 1.
///   - `0b0`: X-axis disabled.
///   - `0b1`: X-axis enabled
pub mod axis_enable {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 3;
    pub const OFFSET: u8 = 0;
    pub type Default = XYZEnabled;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        XYZDisabled = 0b000,
        XEnabled = 0b001,
        YEnabled = 0b010,
        XYEnabled = 0b011,
        ZEnabled = 0b100,
        XZEnabled = 0b101,
        YZEnabled = 0b110,
        XYZEnabled = 0b111,
    }

    macro_rules! impls {
        ($name:ident) => {
            pub struct $name;

            impl State for $name {
                const VARIANT: Variant = Variant::$name;
            }
        };
    }
    impls!(XYZDisabled);
    impls!(XEnabled);
    impls!(YEnabled);
    impls!(XYEnabled);
    impls!(ZEnabled);
    impls!(XZEnabled);
    impls!(YZEnabled);
    impls!(XYZEnabled);
}

define_state_renderer!(odr, lp_en, axis_enable);
