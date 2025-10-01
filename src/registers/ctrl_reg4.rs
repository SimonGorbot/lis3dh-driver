//! # CTRL_REG4 (23h)
//! ## Fields:
//! - `bdu`: Block data update.
//! - `ble`: Big/little endian data selection.
//! - `fs`: Full-scale selection.
//! - `hr`: High-resolution output mode.
//! - `st`: Self-test enable.
//! - `sim`: Spi serial interface mode selection.

use crate::registers::{define_state_renderer, Entitled, ReadWriteRegisterAddress};

pub const ADDR: u8 = ReadWriteRegisterAddress::CtrlReg4 as u8;

/// ### `bdu`: Block data update.
///   - `0b0`: continuous update.
///   - `0b1`: output registers not updated until MSB and LSB reading.
///
/// *Default value: 0 (continuous update).*
pub mod bdu {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 7;
    pub type Default = ContinuousDataUpdate;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        ContinuousDataUpdate = 0b0,
        BlockDataUpdate = 0b1,
    }

    pub struct ContinuousDataUpdate;
    pub struct BlockDataUpdate;

    impl State for ContinuousDataUpdate {
        const VARIANT: Variant = Variant::ContinuousDataUpdate;
    }

    impl State for BlockDataUpdate {
        const VARIANT: Variant = Variant::BlockDataUpdate;
    }
}

/// ### `ble`: Big/little endian data selection.
///   - `0b0`: Data LSB @ lower address (Little Endian).
///   - `0b1`: Data MSB @ lower address (Big Endian).
///
/// *Default value: 0 (Data LSB @ lower address).*
///
/// ### Entitlements:
///    - [`ble::BigEndian`] mode is entitled to [`hr::HighResolution`].
pub mod ble {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 6;
    pub type Default = LittleEndian;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        LittleEndian = 0b0,
        BigEndian = 0b1,
    }

    pub struct BigEndian;
    pub struct LittleEndian;

    impl State for LittleEndian {
        const VARIANT: Variant = Variant::LittleEndian;
    }

    impl State for BigEndian {
        const VARIANT: Variant = Variant::BigEndian;
    }
}

// Entitlements for ble bit-field
impl Entitled<hr::HighResolution> for ble::BigEndian {}

/// ### `fs`: Full-scale selection.
///   - `0b00`: ±2 g.
///   - `0b01`: ±4 g.
///   - `0b10`: ±8 g.
///   - `0b11`: ±16 g.
///
/// *Default value: 00 (±2 g).*
pub mod fs {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 2;
    pub const OFFSET: u8 = 4;
    pub type Default = S2G;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        S2G = 0b00,
        S4G = 0b01,
        S8G = 0b10,
        S16G = 0b11,
    }

    macro_rules! impls {
        ($name:ident) => {
            pub struct $name;

            impl State for $name {
                const VARIANT: Variant = Variant::$name;
            }
        };
    }

    impls!(S2G);
    impls!(S4G);
    impls!(S8G);
    impls!(S16G);
}

/// ### `hr`: High-resolution output mode.
///   - `0`: High-resolution disabled.
///   - `1`: High-resolution enabled.
///
/// *Default value: 0 (disabled).*
///
/// ### Entitlements:
///    - [`hr::HighResolution`] mode is entitled to [`crate::registers::ctrl_reg1::lp_en::LowPowerMode`].
pub mod hr {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 3;
    pub type Default = NormalResolution;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        NormalResolution = 0b0,
        HighResolution = 0b1,
    }

    pub struct NormalResolution;
    pub struct HighResolution;

    impl State for NormalResolution {
        const VARIANT: Variant = Variant::NormalResolution;
    }

    impl State for HighResolution {
        const VARIANT: Variant = Variant::HighResolution;
    }
}

// Entitlements of hr bit field
impl<T: crate::registers::ctrl_reg1::lp_en::State> Entitled<T> for hr::NormalResolution {} // Naming is confusing as in lowpower mode the resolution is 8 bits but that is "normal for that power mode, but at normal power mode and not high res mode the resolution is 10 bits"
impl Entitled<crate::registers::ctrl_reg1::lp_en::NormalPowerMode> for hr::HighResolution {}

/// ### `st`: Self-test enable.
///   - `0b00`: Self-test disabled.
///   - `0b01`: Self-test 0.
///   - `0b10`: Self-test 1.
///   - `0b11`: --.
///
/// *Default value: 00 (disabled).*
pub mod st {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 2;
    pub const OFFSET: u8 = 1;
    pub type Default = NormalMode;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        NormalMode = 0b00,
        SelfTest0 = 0b01,
        SelfTest1 = 0b10,
    }

    pub struct NormalMode;
    pub struct SelfTest0;
    pub struct SelfTest1;

    impl State for NormalMode {
        const VARIANT: Variant = Variant::NormalMode;
    }

    impl State for SelfTest0 {
        const VARIANT: Variant = Variant::SelfTest0;
    }

    impl State for SelfTest1 {
        const VARIANT: Variant = Variant::SelfTest1;
    }
}

/// ### `sim`: SPI serial interface mode selection.
///   - `0b0`: 4-wire interface.
///   - `0b1`: 3-wire interface.
///
/// *Default value: 0 (4-wire interface).*
pub mod sim {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 0;
    pub type Default = Spi4Wire;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        Spi4Wire = 0b0,
        Spi3Wire = 0b1,
    }
    pub struct Spi4Wire;
    pub struct Spi3Wire;

    impl State for Spi4Wire {
        const VARIANT: Variant = Variant::Spi4Wire;
    }

    impl State for Spi3Wire {
        const VARIANT: Variant = Variant::Spi3Wire;
    }
}

define_state_renderer!(bdu, ble, fs, hr, st, sim);
