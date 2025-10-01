use embedded_hal::{self, spi::Operation as EmbeddedHalSpiOperation};
use embedded_hal_async::spi::SpiDevice as EmbeddedHalAsyncSpiDevice;

use crate::bus::Lis3dhBus;
use crate::registers::{ReadWriteRegisterAddress, RegisterAddress};

pub enum Lis3dhOperation {
    SingleWrite = 0b00_000000,
    SingleRead = 0b10_000000,
    // In "Multiple" operations, the address is auto incremented in multiple read/write commands.
    MultipleWrite = 0b01_000000,
    MultipleRead = 0b11_000000,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Lis3dhAsyncSpi<Spi> {
    pub spi: Spi,
}

impl<Spi, ErrSpi> Lis3dhBus for Lis3dhAsyncSpi<Spi>
where
    Spi: EmbeddedHalAsyncSpiDevice<Error = ErrSpi>,
{
    type BusError = ErrSpi;

    async fn write(
        &mut self,
        register_address: ReadWriteRegisterAddress,
        value: u8,
    ) -> Result<(), Self::BusError> {
        let write_buf = [
            Lis3dhOperation::SingleWrite as u8 | register_address as u8,
            value,
        ];
        self.spi
            .transaction(&mut [EmbeddedHalSpiOperation::Write(&write_buf)])
            .await?;
        Ok(())
    }

    async unsafe fn write_multiple(
        &mut self,
        start_address: ReadWriteRegisterAddress,
        values: &[u8],
    ) -> Result<(), Self::BusError> {
        let address_buf = [Lis3dhOperation::MultipleWrite as u8 | start_address as u8];
        self.spi
            .transaction(&mut [
                EmbeddedHalSpiOperation::Write(&address_buf),
                EmbeddedHalSpiOperation::Write(values),
            ])
            .await?;
        Ok(())
    }

    async fn read(
        &mut self,
        register_address: impl Into<RegisterAddress>,
    ) -> Result<u8, Self::BusError> {
        let register_address = register_address.into().byte_address();
        let address_buf: [u8; 1] = [Lis3dhOperation::SingleRead as u8 | register_address];
        let mut result_buf: [u8; 1] = [0u8];
        self.spi
            .transaction(&mut [
                EmbeddedHalSpiOperation::Write(&address_buf),
                EmbeddedHalSpiOperation::Read(&mut result_buf),
            ])
            .await?;
        Ok(result_buf[0])
    }

    async fn read_multiple(
        &mut self,
        start_address: impl Into<RegisterAddress>,
        result: &mut [u8],
    ) -> Result<(), Self::BusError> {
        let start_address = start_address.into().byte_address();
        let address_buf = [Lis3dhOperation::MultipleRead as u8 | start_address];
        self.spi
            .transaction(&mut [
                EmbeddedHalSpiOperation::Write(&address_buf),
                EmbeddedHalSpiOperation::Read(result),
            ])
            .await?;
        Ok(())
    }
}
