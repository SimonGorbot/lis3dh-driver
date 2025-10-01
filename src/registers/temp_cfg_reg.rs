//! # TEMP_CFG_REG (1Fh)
//! ## Fields:
//! - `adc_en`: ADC enable.
//! - `temp_en`: Temperature sensor (T) enable.

use crate::registers::{define_state_renderer, ReadWriteRegisterAddress};

pub const ADDR: u8 = ReadWriteRegisterAddress::TempCfgReg as u8;

/// ### `adc_en`: ADC enable.
///   - `0b0`: ADC disabled.
///   - `0b1`: ADC enabled.
///
/// *Default value: 0 (ADC disabled).*
pub mod adc_en {
    pub const ADDR: u8 = super::ReadWriteRegisterAddress::TempCfgReg as u8;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 7;
    pub type Default = AdcDisabled;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        AdcDisabled = 0b0,
        AdcEnabled = 0b1,
    }

    pub struct AdcDisabled;
    pub struct AdcEnabled;

    impl State for AdcDisabled {
        const VARIANT: Variant = Variant::AdcDisabled;
    }

    impl State for AdcEnabled {
        const VARIANT: Variant = Variant::AdcEnabled;
    }
}

/// ### `temp_en`: Temperature sensor (T) enable.
///   - `0b0`: T disabled.
///   - `0b1`: T enabled.
///
/// *Default value: 0 (T disabled).*
pub mod temp_en {
    pub const ADDR: u8 = super::ADDR;
    pub const WIDTH: u8 = 1;
    pub const OFFSET: u8 = 6;
    pub type Default = TempDisabled;

    pub trait State {
        const VARIANT: Variant;
    }

    #[repr(u8)]
    pub enum Variant {
        TempDisabled = 0b0,
        TempEnabled = 0b1,
    }

    pub struct TempDisabled;
    pub struct TempEnabled;

    impl State for TempDisabled {
        const VARIANT: Variant = Variant::TempDisabled;
    }

    impl State for TempEnabled {
        const VARIANT: Variant = Variant::TempEnabled;
    }
}

define_state_renderer!(adc_en, temp_en);
