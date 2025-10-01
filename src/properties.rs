//! # Properties
//! Properties are values that can be derived from the hardware-states of the lis3dh.

/// # Resolution
/// LIS3DH provides three different operating modes: high-resolution mode (12-bit), normal mode (10-bit) and low-power mode (8-bit). The resolution of the acceleration readings are a result of configuration of the [`crate::registers::ctrl_reg1::lp_en`] and [`crate::registers::ctrl_reg4::hr`] bit-fields.
///
/// | [`resolution`]                            | [`crate::registers::ctrl_reg1::lp_en`] | [`crate::registers::ctrl_reg4::hr`] |
/// |-------------------------------------------|:-----------------------:|:---------------------:|
/// | Low-power mode (8-bit data output)        |            1            |           0           |
/// | Normal mode (10-bit data output)          |            0            |           0           |
/// | High-resolution mode (12-bit data output) |            0            |           1           |
/// | Not allowed                               |            1            |           1           |
pub mod resolution {
    #[derive(PartialEq)]
    #[repr(u8)]
    pub enum Variant {
        R8Bit = 8,
        R10Bit = 10,
        R12Bit = 12,
    }

    pub trait Property {
        const VARIANT: Variant;
    }

    pub struct Resolution<LpEn, Hr>
    where
        Hr: crate::registers::ctrl_reg4::hr::State,
        LpEn: crate::registers::ctrl_reg1::lp_en::State,
    {
        _p: core::marker::PhantomData<(LpEn, Hr)>,
    }

    impl<LpEn, Hr> Property for Resolution<LpEn, Hr>
    where
        Hr: crate::registers::ctrl_reg4::hr::State,
        LpEn: crate::registers::ctrl_reg1::lp_en::State,
    {
        const VARIANT: Variant = {
            use crate::registers::ctrl_reg1::lp_en;
            use crate::registers::ctrl_reg4::hr;
            match (LpEn::VARIANT, Hr::VARIANT) {
                (lp_en::Variant::LowPowerMode, hr::Variant::NormalResolution) => Variant::R8Bit,
                (lp_en::Variant::NormalPowerMode, hr::Variant::NormalResolution) => Variant::R10Bit,
                (lp_en::Variant::NormalPowerMode, hr::Variant::HighResolution) => Variant::R12Bit,
                (lp_en::Variant::LowPowerMode, hr::Variant::HighResolution) => unreachable!(),
            }
        };
    }
}

/// # Gravity Coefficient
/// The coefficient applied to the raw 2's compliment acceleration to obtain a value in units g is a result of the configuration of bit-field [`crate::registers::ctrl_reg4::fs`] and property [`resolution`].
///
/// | Full Scale ([`crate::registers::ctrl_reg4::fs`]) | Resolution ([`resolution`]) | [`gravity_coefficient`] (g/digit) |
/// |:-----------------------------------:|:---------------------------:|:-------------------:|
/// | `S2G`                               | `R8Bit`                     | 0.016               |
/// | `S2G`                               | `R10Bit`                    | 0.004               |
/// | `S2G`                               | `R12Bit`                    | 0.001               |
/// | `S4G`                               | `R8Bit`                     | 0.032               |
/// | `S4G`                               | `R10Bit`                    | 0.008               |
/// | `S4G`                               | `R12Bit`                    | 0.002               |
/// | `S8G`                               | `R8Bit`                     | 0.064               |
/// | `S8G`                               | `R10Bit`                    | 0.016               |
/// | `S8G`                               | `R12Bit`                    | 0.004               |
/// | `S16G`                              | `R8Bit`                     | 0.192               |
/// | `S16G`                              | `R10Bit`                    | 0.048               |
/// | `S16G`                              | `R12Bit`                    | 0.012               |
pub mod gravity_coefficient {

    pub trait Property {
        const GRAVITY_COEFFICIENT: f32;
    }

    pub struct GravityCoefficient<Fs, Res>
    where
        Fs: crate::registers::ctrl_reg4::fs::State,
        Res: super::resolution::Property,
    {
        _p: core::marker::PhantomData<(Fs, Res)>,
    }

    impl<Fs, Resolution> Property for GravityCoefficient<Fs, Resolution>
    where
        Fs: crate::registers::ctrl_reg4::fs::State,
        Resolution: super::resolution::Property,
    {
        const GRAVITY_COEFFICIENT: f32 = {
            use crate::registers::ctrl_reg4::fs;
            match (Fs::VARIANT, Resolution::VARIANT) {
                (fs::Variant::S2G, super::resolution::Variant::R8Bit) => 0.016,
                (fs::Variant::S2G, super::resolution::Variant::R10Bit) => 0.004,
                (fs::Variant::S2G, super::resolution::Variant::R12Bit) => 0.001,

                (fs::Variant::S4G, super::resolution::Variant::R8Bit) => 0.032,
                (fs::Variant::S4G, super::resolution::Variant::R10Bit) => 0.008,
                (fs::Variant::S4G, super::resolution::Variant::R12Bit) => 0.002,

                (fs::Variant::S8G, super::resolution::Variant::R8Bit) => 0.064,
                (fs::Variant::S8G, super::resolution::Variant::R10Bit) => 0.016,
                (fs::Variant::S8G, super::resolution::Variant::R12Bit) => 0.004,

                (fs::Variant::S16G, super::resolution::Variant::R8Bit) => 0.192,
                (fs::Variant::S16G, super::resolution::Variant::R10Bit) => 0.048,
                (fs::Variant::S16G, super::resolution::Variant::R12Bit) => 0.012,
            }
        };
    }
}
