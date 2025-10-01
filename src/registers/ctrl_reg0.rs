//! # CTRL_REG0 (1Eh)
//! ## Fields:
//! - `sdo_pu_disc`: Disconnect SDO/SA0 pull-up.
//!
//! **Note:** register must have the following bits set, otherwise the sensor may exhibit undefined behaviour:
//! 0b0010000

use crate::registers::{define_state_renderer, ReadWriteRegisterAddress};

pub const ADDR: u8 = ReadWriteRegisterAddress::CtrlReg0 as u8;

/// ### `SDO_PU_DISC`: Disconnect SDO/SA0 pull-up.
///   - `0b0`: pull-up connected to SDO/SA0 pin.
///   - `0b1`: pull-up disconnected from SDO/SA0 pin.
///
/// *Default value: 0 (pull-up connected to SDO/SA0 pin).*
pub mod sdo_pu_disc {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 7;
    pub type Default = SdoPulledUp;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        SdoPulledUp = 0b0,
        SdoFloating = 0b1,
    }

    pub struct SdoPulledUp;
    pub struct SdoFloating;

    impl State for SdoPulledUp {
        const VARIANT: Variant = Variant::SdoPulledUp;
    }

    impl State for SdoFloating {
        const VARIANT: Variant = Variant::SdoFloating;
    }
}

/// CTRL_REG0 8-bit register must have the following bits set for correct operation of the device: 0b0010000 as per datasheet pg. 34
pub mod must_set_bits {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 7;
    pub const OFFSET: u8 = 0;
    pub type Default = MustSet;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        MustSet = 0b0010000,
    }

    pub struct MustSet;

    impl State for MustSet {
        const VARIANT: Variant = Variant::MustSet;
    }
}

define_state_renderer!(sdo_pu_disc, must_set_bits);
