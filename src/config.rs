use crate::properties::*;
use crate::registers::*;

// Necessary functionality (for Bombus) can be achieved by only configuring ctrl_reg1 and ctrl_reg4.
// TODO: Add all additional functionality to Config.
pub struct Config<Odr, LpEn, AxisEnable, Fs, Hr>
where
    Odr: ctrl_reg1::odr::State + Entitled<LpEn>,
    LpEn: ctrl_reg1::lp_en::State,
    AxisEnable: ctrl_reg1::axis_enable::State,
    Fs: ctrl_reg4::fs::State,
    Hr: ctrl_reg4::hr::State + Entitled<LpEn>,
{
    pub data_rate: Odr,
    pub power_mode: LpEn,
    pub axis_enable: AxisEnable,
    pub full_scale: Fs,
    pub resolution_mode: Hr,
}

/// The register values represented by some [`ValidLis3dhConfig`].
pub struct ConfigAsBytes {
    pub(crate) ctrl_reg0: u8,
    pub(crate) temp_cfg_reg: u8,
    pub(crate) ctrl_reg1: u8,
    pub(crate) ctrl_reg4: u8,
    // More registers to come...
}

mod sealed {
    pub trait Sealed {}
}

/// `ValidLis3dhConfig` is a convenience trait that provides a way to pass **valid** generic lis3dh configurations, rather than concrete configurations. If this trait didn't exist, one would need to specify the many generic parameters of config, but with the trait, `where T: ValidLis3dhConfig` can be used.
pub trait ValidLis3dhConfig: sealed::Sealed {
    // Type-states corresponding to lis3dh Config and entitlement check.
    type Odr: ctrl_reg1::odr::State + Entitled<Self::LpEn>;
    type LpEn: ctrl_reg1::lp_en::State;
    type AxisEnable: ctrl_reg1::axis_enable::State;
    type Fs: ctrl_reg4::fs::State;
    type Hr: ctrl_reg4::hr::State + Entitled<Self::LpEn>;

    // Properties corresponding to lis3dh Config.
    type Resolution: resolution::Property;
    type GravityCoefficient: gravity_coefficient::Property;

    /// Render some [`ValidLis3dhConfig`] to bytes.
    fn render_as_bytes() -> ConfigAsBytes;
}

impl<Odr, LpEn, AxisEnable, Fs, Hr> sealed::Sealed for Config<Odr, LpEn, AxisEnable, Fs, Hr>
where
    Odr: ctrl_reg1::odr::State + Entitled<LpEn>,
    LpEn: ctrl_reg1::lp_en::State,
    AxisEnable: ctrl_reg1::axis_enable::State,
    Fs: ctrl_reg4::fs::State,
    Hr: ctrl_reg4::hr::State + Entitled<LpEn>,
{
}

// TODO: Create helper traits per register to improve readability and reduce number of generic parameters.
impl<Odr, LpEn, AxisEnable, Fs, Hr> ValidLis3dhConfig for Config<Odr, LpEn, AxisEnable, Fs, Hr>
where
    Odr: ctrl_reg1::odr::State + Entitled<LpEn>,
    LpEn: ctrl_reg1::lp_en::State,
    AxisEnable: ctrl_reg1::axis_enable::State,
    Fs: ctrl_reg4::fs::State,
    Hr: ctrl_reg4::hr::State + Entitled<LpEn>,
{
    // Type-States
    type Odr = Odr;
    type LpEn = LpEn;
    type AxisEnable = AxisEnable;
    type Fs = Fs;
    type Hr = Hr;

    // Resulting Properties:
    type Resolution = resolution::Resolution<Self::LpEn, Self::Hr>;
    type GravityCoefficient = gravity_coefficient::GravityCoefficient<Self::Fs, Self::Resolution>;

    fn render_as_bytes() -> ConfigAsBytes {
        ConfigAsBytes {
            ctrl_reg0: ctrl_reg0::render_hardware_state::<
                ctrl_reg0::sdo_pu_disc::Default,
                ctrl_reg0::must_set_bits::Default,
            >(),
            temp_cfg_reg: temp_cfg_reg::render_hardware_state::<
                temp_cfg_reg::adc_en::Default,
                temp_cfg_reg::temp_en::Default,
            >(),
            ctrl_reg1: ctrl_reg1::render_hardware_state::<Odr, LpEn, AxisEnable>(),
            ctrl_reg4: ctrl_reg4::render_hardware_state::<
                ctrl_reg4::bdu::Default,
                ctrl_reg4::ble::Default,
                Fs,
                Hr,
                ctrl_reg4::st::Default,
                ctrl_reg4::sim::Default,
            >(),
        }
    }
}
