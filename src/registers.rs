//! # Registers
//! The hardware states in the fields of registers are represented as type-states.
//! The use of type-states in the crate is heavily inspired by work done by AdinAck in his pursuit of creating tools to generate better HALs.
//! Read more about Adin's work here: <https://adinack.dev/blog/better-hals-first-look/>
//!
//!  Type-state Structure Description:
//!  Type-states are marker types that directly correspond to hardware states.
//!  Hardware states are in the form of register bit-fields. The value placed in the bit-field that results in a specific type-state is known as the raw value. Possible hardware states and their raw values are defined as variants in an enum named `Variant`.
//!  Type-states will implement a trait named State which contains a constant VARIANT of the type Variant and value corresponding to the raw value to achieve the represented hardware state.
//!
//!  Consider:
//!  A device has multiple sensors. Sensor 1 can be enabled or disabled, as well as have its measurement range changed. This chip is very picky though, and if the sensor is disabled, it's range must be set to a specific value otherwise it exhibits undefined behaviour.
//!  ```
//!  pub mod sensor_1_enable {
//!     pub trait State {
//!         const VARIANT: Variant;
//!     }
//!     
//!     pub enum Variant {
//!         SensorEnabled = 0b0,
//!         SensorDisabled = 0b1,
//!     }
//!
//!     pub struct SensorEnabled;
//!     pub struct SensorDisabled;
//!
//!     impl State for SensorEnabled {
//!         const VARIANT: Variant = Variant::SensorEnabled;
//!     }
//!
//!     impl State for SensorDisabled {
//!         const VARIANT: Variant = Variant::SensorDisabled;
//!     }
//! }
//!
//!  pub mod sensor_1_range {
//!     pub trait State {
//!         const VARIANT: Variant;
//!     }
//!     
//!     pub enum Variant {
//!         RangeDisabled = 0b00,
//!         Range1 = 0b01,
//!         Range2 = 0b10,
//!         Range3 = 0b11,
//!     }
//!
//!     pub struct RangeDisabled; // When sensor 1 is disabled, value of range must be 0b00, otherwise undefined behaviour.
//!     pub struct Range1;
//!     pub struct Range2;
//!     pub struct Range3;
//!
//!     impl State for RangeDisabled {
//!         const VARIANT: Variant = Variant::RangeDisabled;
//!     }
//!
//!     impl State for Range1 {
//!         const VARIANT: Variant = Variant::Range1;
//!     }
//!
//!     impl State for Range2 {
//!         const VARIANT: Variant = Variant::Range2;
//!     }
//!
//!      impl State for Range3 {
//!         const VARIANT: Variant = Variant::Range1;
//!     }
//! }
//! ```
//!
//! The Entitled trait is used to express inter-bit-field relationships in the type system.
//! For example, if sensor_1_range can only be set to one of the options Range1, Range2, and Range3, if sensor_1_enable is set to SensorEnabled, then one could say that the Type-States Range1, Range2, and Range3 of sensor_1_resolution are Entitled to the Type-State SensorEnabled of sensor_1_enable.
//! The mandatory hardware state of the sensor range bit-field when the sensor is disabled can be enforced by the compiler using Entitlements.
//!
//! In code this would look like:
//! ```
//!  // The compiler enforces that sensor 1 can only be disabled if the sensor range is set to disabled.
//!
//!  impl Entitled<sensor_1_range::RangeDisabled> for sensor_1_enable::SensorDisabled {}
//!
//!  // The rest of the ranges, naturally, require the sensor to be enabled.
//!  impl Entitled<sensor_1_enable::SensorEnabled> for sensor_1_range::Range1 {}
//!  impl Entitled<sensor_1_enable::SensorEnabled> for sensor_1_range::Range2 {}
//!  impl Entitled<sensor_1_enable::SensorEnabled> for sensor_1_range::Range3 {}
//!  ```
//!
//!  In summary,
//!  Type-states express hardware states.
//!  The type relationships (as expressed by the `Entitled` trait) provide a proxy for the true hardware relationships.
//!  The resulting structures facilitate correct hardware usage.

// -----------------------------------------------------------------------------
// | Name                   | Type | Address (Hex) | Address (Binary) | Default    | Comment                                   |
// -----------------------------------------------------------------------------
// Reserved (do not modify)   N/A    00 - 06         N/A                N/A          Reserved addresses; do not modify.
// STATUS_REG_AUX             r      07              000 0111           N/A          Auxiliary status register output.
// OUT_ADC1_L                 r      08              000 1000           N/A          Output: Lower byte of ADC channel 1 data.
// OUT_ADC1_H                 r      09              000 1001           N/A          Output: Higher byte of ADC channel 1 data.
// OUT_ADC2_L                 r      0A              000 1010           N/A          Output: Lower byte of ADC channel 2 data.
// OUT_ADC2_H                 r      0B              000 1011           N/A          Output: Higher byte of ADC channel 2 data.
// OUT_ADC3_L                 r      0C              000 1100           N/A          Output: Lower byte of ADC channel 3 data.
// OUT_ADC3_H                 r      0D              000 1101           N/A          Output: Higher byte of ADC channel 3 data.
// Reserved (do not modify)   N/A    0E              N/A                N/A          Reserved address; do not modify.
// WHO_AM_I                   r      0F              000 1111           00110011     Device identification register (dummy).
// Reserved (do not modify)   N/A    10 - 1D         N/A                N/A          Reserved addresses; do not modify.
// CTRL_REG0                  rw     1E              001 1110           00010000     Control register 0.
// TEMP_CFG_REG               rw     1F              001 1111           00000000     Temperature sensor configuration register.
// CTRL_REG1                  rw     20              010 0000           00000111     Control register 1.
// CTRL_REG2                  rw     21              010 0001           00000000     Control register 2.
// CTRL_REG3                  rw     22              010 0010           00000000     Control register 3.
// CTRL_REG4                  rw     23              010 0011           00000000     Control register 4.
// CTRL_REG5                  rw     24              010 0100           00000000     Control register 5.
// CTRL_REG6                  rw     25              010 0101           00000000     Control register 6.
// REFERENCE                  rw     26              010 0110           00000000     Reference value register.
// STATUS_REG                 r      27              010 0111           N/A          Main status register output.
// OUT_X_L                    r      28              010 1000           N/A          Output: Lower byte of X-axis acceleration data.
// OUT_X_H                    r      29              010 1001           N/A          Output: Higher byte of X-axis acceleration data.
// OUT_Y_L                    r      2A              010 1010           N/A          Output: Lower byte of Y-axis acceleration data.
// OUT_Y_H                    r      2B              010 1011           N/A          Output: Higher byte of Y-axis acceleration data.
// OUT_Z_L                    r      2C              010 1100           N/A          Output: Lower byte of Z-axis acceleration data.
// OUT_Z_H                    r      2D              010 1101           N/A          Output: Higher byte of Z-axis acceleration data.
// FIFO_CTRL_REG              rw     2E              010 1110           00000000     FIFO control register.
// FIFO_SRC_REG               r      2F              010 1111           N/A          FIFO source register output.
// INT1_CFG                   rw     30              011 0000           00000000     Interrupt 1 configuration register.
// INT1_SRC                   r      31              011 0001           N/A          Interrupt 1 source register output.
// INT1_THS                   rw     32              011 0010           00000000     Interrupt 1 threshold register.
// INT1_DURATION              rw     33              011 0011           00000000     Interrupt 1 duration register.
// INT2_CFG                   rw     34              011 0100           00000000     Interrupt 2 configuration register.
// INT2_SRC                   r      35              011 0101           N/A          Interrupt 2 source register output.
// INT2_THS                   rw     36              011 0110           00000000     Interrupt 2 threshold register.
// INT2_DURATION              rw     37              011 0111           00000000     Interrupt 2 duration register.
// CLICK_CFG                  rw     38              011 1000           00000000     Click interrupt configuration register.
// CLICK_SRC                  r      39              011 1001           N/A          Click interrupt source register output.
// CLICK_THS                  rw     3A              011 1010           00000000     Click interrupt threshold register.
// TIME_LIMIT                 rw     3B              011 1011           00000000     Click interrupt time limit register.
// TIME_LATENCY               rw     3C              011 1100           00000000     Click interrupt time latency register.
// TIME_WINDOW                rw     3D              011 1101           00000000     Click interrupt time window register.
// ACT_THS                    rw     3E              011 1110           00000000     Activity interrupt threshold register.
// ACT_DUR                    rw     3F              011 1111           00000000

pub mod ctrl_reg0;
pub mod ctrl_reg1;
pub mod ctrl_reg4;
pub mod temp_cfg_reg;

// Register Addresses
pub enum ReadWriteRegisterAddress {
    /// CTRL_REG0
    CtrlReg0 = 0x1E,
    /// TEMP_CFG_REG
    TempCfgReg = 0x1F,
    /// CTRL_REG1
    CtrlReg1 = 0x20,
    /// CTRL_REG2
    CtrlReg2 = 0x21,
    /// CTRL_REG3
    CtrlReg3 = 0x22,
    /// CTRL_REG4
    CtrlReg4 = 0x23,
    /// CTRL_REG5
    CtrlReg5 = 0x24,
    /// CTRL_REG6
    CtrlReg6 = 0x25,
    /// FIFO_CTRL_REG
    FifoCtrlReg = 0x2E,
    /// INT1_CFG
    Int1Cfg = 0x30,
    /// INT1_THS
    Int1Ths = 0x32,
    /// INT1_DURATION
    Int1Duration = 0x33,
    /// INT2_CFG
    Int2Cfg = 0x34,
    /// INT2_THS
    Int2Ths = 0x36,
    /// INT2_DURATION
    Int2Duration = 0x37,
    /// CLICK_CFG
    ClickCfg = 0x38,
    /// CLICK_THS
    ClickThs = 0x3A,
    /// TIME_LIMIT
    TimeLimit = 0x3B,
    /// TIME_LATENCY
    TimeLatency = 0x3C,
    /// TIME_WINDOW
    TimeWindow = 0x3D,
    /// ACT_THS
    ActThs = 0x3E,
    /// ACT_DUR
    ActDur = 0x3F,
}

pub enum ReadOnlyRegisterAddress {
    /// STATUS_REG_AUX
    StatusRegAux = 0x07,
    /// OUT_ADC1_L
    OutAdc1L = 0x08,
    /// OUT_ADC1_H
    OutAdc1H = 0x09,
    /// OUT_ADC2_L
    OutAdc2L = 0x0A,
    /// OUT_ADC2_H
    OutAdc2H = 0x0B,
    /// OUT_ADC3_L
    OutAdc3L = 0x0C,
    /// OUT_ADC3_H
    OutAdc3H = 0x0D,
    /// WHO_AM_I
    WhoAmI = 0x0F,
    /// REFERENCE
    Reference = 0x26,
    /// STATUS_REG
    StatusReg = 0x27,
    /// OUT_X_L
    OutXL = 0x28,
    /// OUT_X_H
    OutXH = 0x29,
    /// OUT_Y_L
    OutYL = 0x2A,
    /// OUT_Y_H
    OutYH = 0x2B,
    /// OUT_Z_L
    OutZL = 0x2C,
    /// OUT_Z_H
    OutZH = 0x2D,
    /// FIFO_SRC_REG
    FifoSrcReg = 0x2F,
    /// INT1_SRC
    Int1Src = 0x31,
    /// INT2_SRC
    Int2Src = 0x35,
    /// CLICK_SRC
    ClickSrc = 0x39,
}

pub enum RegisterAddress {
    ReadOnly(ReadOnlyRegisterAddress),
    ReadWrite(ReadWriteRegisterAddress),
}

impl From<ReadOnlyRegisterAddress> for RegisterAddress {
    fn from(value: ReadOnlyRegisterAddress) -> Self {
        Self::ReadOnly(value)
    }
}

impl From<ReadWriteRegisterAddress> for RegisterAddress {
    fn from(value: ReadWriteRegisterAddress) -> Self {
        Self::ReadWrite(value)
    }
}

impl RegisterAddress {
    pub fn byte_address(self) -> u8 {
        match self {
            RegisterAddress::ReadOnly(register_address) => register_address as u8,
            RegisterAddress::ReadWrite(register_address) => register_address as u8,
        }
    }
}

// The Entitled trait is used to express inter-bit-field relationships to the compiler.
pub trait Entitled<T> {}

/// Macro that takes the corresponding register's field modules and creates the function `render_hardware_state`. The function takes the fields' type-state as type parameters and renders them to a single byte to be written to the corresponding register.
macro_rules! define_state_renderer {
    (
        $( $module:ident ),+
    ) => {
        paste::paste!{
            #[doc = "Render `" $($module) "`, `" + "` fields from type-states to single byte (hardware-state) to be written to register."]
            pub(crate) fn render_hardware_state < $( [<$module:camel>] ),+ >() -> u8
            where
                // Create "where" bound for each Type-State.
                $( [<$module:camel>] : $module::State ),+
            {
                // Create the bitwise OR chain for the function body
                $(
                    (([<$module:camel>]::VARIANT as u8) << $module::OFFSET)
                )|+
            }
        }
    };
}

pub(crate) use define_state_renderer;
