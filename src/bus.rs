pub mod spi;

use core::future::Future;

use crate::registers::{ReadWriteRegisterAddress, RegisterAddress};

/// `Lis3dhCore` allows for the `Lis3dh` type to be bus agnostic, so long as the bus implements `Lis3dhCore`.
pub trait Lis3dhBus {
    type BusError;

    /// Write a single register value to the lis3dh.
    fn write(
        &mut self,
        register_address: ReadWriteRegisterAddress,
        value: u8,
    ) -> impl Future<Output = Result<(), Self::BusError>>;

    /// Write multiple consecutive register values to the lis3dh. The address and `values` index is incremented by 1 then written for every byte in the write buffer passed.
    /// # Safety
    /// This function does not check if all registers being broadcast to are writable so you **must** guarantee registers in the broadcast are safe to write to.
    unsafe fn write_multiple(
        &mut self,
        start_address: ReadWriteRegisterAddress,
        values: &[u8],
    ) -> impl Future<Output = Result<(), Self::BusError>>;

    /// Read a single register value from the lis3dh.
    fn read(
        &mut self,
        register_address: impl Into<RegisterAddress>,
    ) -> impl Future<Output = Result<u8, Self::BusError>>;

    /// Read multiple consecutive register values from the lis3dh. The address is incremented by 1 and read into the result buffer passed until full.
    fn read_multiple(
        &mut self,
        start_address: impl Into<RegisterAddress>,
        result: &mut [u8],
    ) -> impl Future<Output = Result<(), Self::BusError>>;

    /// Reads a single register value from the lis3dh and returns true if the value is equal to the expected result and false otherwise.
    fn read_and_verify(
        &mut self,
        address: impl Into<RegisterAddress>,
        expected_result: &u8,
    ) -> impl Future<Output = Result<bool, Self::BusError>> {
        async { Ok(self.read(address).await? == *expected_result) }
    }
}
