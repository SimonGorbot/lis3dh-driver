#![no_std]
#![no_main]

pub mod acceleration_data_structs;
pub mod bus;
pub mod config;
pub mod properties;
pub mod registers;

use crate::acceleration_data_structs::{Acceleration, AccelerationVector};
use crate::bus::Lis3dhBus;
use crate::config::ValidLis3dhConfig;
use crate::properties::resolution;
use crate::registers::{ReadOnlyRegisterAddress, ReadWriteRegisterAddress, RegisterAddress};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<BusErrorType> {
    /// # Bus error
    /// An error originating from the bus communication method (I2C or SPI) used as the communication method between the controller and the Lis3dh.
    Bus(BusErrorType),
}

impl<BusErrorType> From<BusErrorType> for Error<BusErrorType> {
    fn from(error: BusErrorType) -> Self {
        Error::Bus(error)
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(not(feature = "defmt"), expect(unused))]
pub struct Lis3dh<Bus, Config>
where
    Bus: Lis3dhBus,
    Config: config::ValidLis3dhConfig,
{
    bus: Bus,
    config: Config, // Maybe needed in the future.
}

impl<Bus, Config> Lis3dh<Bus, Config>
where
    Bus: Lis3dhBus,
    Config: ValidLis3dhConfig,
{
    pub async fn new(mut bus: Bus, config: Config) -> Result<Self, Error<Bus::BusError>> {
        let config::ConfigAsBytes {
            ctrl_reg0: ctrl_reg0_bytes,
            temp_cfg_reg: temp_cfg_reg_bytes,
            ctrl_reg1: ctrl_reg1_bytes,
            ctrl_reg4: ctrl_reg4_bytes,
        } = Config::render_as_bytes();

        // Write Block 1: CtrlReg0 (0x1E) to CtrlReg1 (0x20)
        let config_write_block_ctrl_reg0_to_ctrl_reg1 =
            [ctrl_reg0_bytes, temp_cfg_reg_bytes, ctrl_reg1_bytes];

        // SAFETY: Starting memory address `CtrlReg0 = 0x1E` incremented 2 times leads to `CtrlReg1 = 0x20` which are all writable memory addresses.
        unsafe {
            bus.write_multiple(
                ReadWriteRegisterAddress::CtrlReg0,
                &config_write_block_ctrl_reg0_to_ctrl_reg1,
            )
            .await?
        };

        // Write Block 2: CtrlReg4 (0x23)
        bus.write(ReadWriteRegisterAddress::CtrlReg4, ctrl_reg4_bytes)
            .await?;

        Ok(Lis3dh { bus, config })
    }

    // For now reconfiguration of the lis3dh will be done by re-writing the entire config in the interest of time and implementation priority as it's a niche scenario to require a more optimized re-configuration.
    pub async fn reconfigure<NewConfig>(
        self,
        new_config: NewConfig,
    ) -> Result<Lis3dh<Bus, NewConfig>, Error<Bus::BusError>>
    where
        NewConfig: ValidLis3dhConfig,
    {
        Lis3dh::new(self.bus, new_config).await
    }

    pub async fn read_who_am_i(&mut self) -> Result<u8, Error<Bus::BusError>> {
        Ok(self.bus.read(ReadOnlyRegisterAddress::WhoAmI).await?)
    }

    /// Convenience function to perform the combination of lower & upper acceleration values then adjusts based on configured resolution.
    fn accel_raw_into_i16(lower_byte: u8, upper_byte: u8) -> i16 {
        let accel_as_i16 = i16::from_le_bytes([lower_byte, upper_byte]);
        accel_as_i16 >> (16 - <Config::Resolution as resolution::Property>::VARIANT as u8)
    }

    /// Reads and returns the acceleration values from `OUT_X_L (0x28)` to `OUT_Z_U (0x2D)`
    pub async fn read_accel_bytes(&mut self) -> Result<[u8; 6], Error<Bus::BusError>> {
        let mut result = [0; 6];
        self.bus
            .read_multiple(ReadOnlyRegisterAddress::OutXL, &mut result)
            .await?;
        Ok(result)
    }

    /// Returns the resolution adjusted signed integer value from concatenated upper and lower bytes for each acceleration axis.
    pub async fn get_accel_vector(&mut self) -> Result<AccelerationVector, Error<Bus::BusError>> {
        let [a_x_l, a_x_u, a_y_l, a_y_u, a_z_l, a_z_u] = self.read_accel_bytes().await?;
        let x = Acceleration::new(Self::accel_raw_into_i16(a_x_l, a_x_u));
        let y = Acceleration::new(Self::accel_raw_into_i16(a_y_l, a_y_u));
        let z = Acceleration::new(Self::accel_raw_into_i16(a_z_l, a_z_u));
        Ok(AccelerationVector { x, y, z })
    }
}

// Register read/write commands.

impl<Bus, Config> Lis3dh<Bus, Config>
where
    Bus: Lis3dhBus,
    Config: ValidLis3dhConfig,
{
    pub async fn read_register(
        &mut self,
        register_address: impl Into<RegisterAddress>,
    ) -> Result<u8, Error<Bus::BusError>> {
        Ok(self.bus.read(register_address).await?)
    }

    /// Read multiple consecutive register values from the lis3dh. The address is incremented by 1 then read for every byte in the read buffer passed.
    /// # Safety
    /// This function does not check if all registers addresses being read are valid. Attempting to read from invalid addresses may lead to undefined behaviour.
    pub async unsafe fn read_multiple_registers(
        &mut self,
        start_address: impl Into<RegisterAddress>,
        result: &mut [u8],
    ) -> Result<(), Error<Bus::BusError>> {
        Ok(self.bus.read_multiple(start_address, result).await?)
    }

    /// Write a single value to a given register of the lis3dh.
    /// # Safety
    /// There is no check check for the validity of the byte being written to the specified register. Invalid register configurations may lead to undefined behaviour.
    pub async unsafe fn write_register(
        &mut self,
        register_address: ReadWriteRegisterAddress,
        value: u8,
    ) -> Result<(), Error<Bus::BusError>> {
        Ok(self.bus.write(register_address, value).await?)
    }

    /// Write multiple consecutive register values to the lis3dh. The address and `values` index is incremented by 1 then written for every byte in the write buffer passed.
    /// # Safety
    /// This function does not check if all registers being broadcast to are writable so you **must** guarantee registers in the broadcast are safe to write to.
    /// There is no check check for the validity of the bytes being written to registers. Invalid register configurations may lead to undefined behaviour.
    pub async unsafe fn write_multiple_registers(
        &mut self,
        start_address: ReadWriteRegisterAddress,
        values: &mut [u8],
    ) -> Result<(), Error<Bus::BusError>> {
        Ok(self.bus.write_multiple(start_address, values).await?)
    }
}

mod sealed {
    pub trait Sealed {}
}

/// `Lis3dhTypes` is a trait that provides convenient access to related types of the Lis3dh struct. As such, users of Lis3dh don't have to write out complex error, config, and bus types for a given Lis3dh struct.
pub trait Lis3dhTypes: sealed::Sealed {
    type Bus: Lis3dhBus;
    type BusError;
    type Config: ValidLis3dhConfig;
}

impl<Bus: Lis3dhBus, Config: ValidLis3dhConfig> sealed::Sealed for Lis3dh<Bus, Config> {}

impl<Bus: Lis3dhBus, Config: ValidLis3dhConfig> Lis3dhTypes for Lis3dh<Bus, Config> {
    type Bus = Bus;
    type BusError = Bus::BusError;
    type Config = Config;
}
