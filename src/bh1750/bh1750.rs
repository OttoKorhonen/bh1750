use crate::bh1750::Bh1750Error;
use crate::bh1750::Command;
use core::borrow::BorrowMut;
use core::error::Error;
use core::fmt;
use embedded_hal::{delay::DelayNs, i2c::SevenBitAddress};

pub struct Bh1750<I2C, D> {
    i2c: I2C,
    address: SevenBitAddress,
    delay: D,
}

impl<E: fmt::Debug> Error for Bh1750Error<E> {}

impl<I2C, D> Bh1750<I2C, D>
where
    I2C: embedded_hal::i2c::I2c,
    I2C::Error: fmt::Debug,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: u8, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

     pub fn init(&mut self) -> Result<(), Bh1750Error<I2C::Error>> {
        self.power_on()?;
        self.delay.delay_ms(10);
        self.reset()?;
        self.delay.delay_ms(10);
        Ok(())
    }

    pub fn read(&mut self, command: Command) -> Result<u16, Bh1750Error<I2C::Error>> {
        self.i2c
            .write(self.address, &[command as u8])
            .map_err(Bh1750Error::I2cError)?;

        self.delay.delay_ms(120);

        let mut result = [0u8; 2];
        self.i2c
            .read(self.address, &mut result)
            .map_err(Bh1750Error::I2cError)?;

        Ok(u16::from_be_bytes(result))
    }

    ///Waiting for measurement command.
    pub fn power_on(&mut self) -> Result<(), Bh1750Error<I2C::Error>> {
        let data = 0b0000_0001;
        self.i2c
            .borrow_mut()
            .write(self.address, &[data])
            .map_err(Bh1750Error::I2cError)?;
        Ok(())
    }
    ///Reset Data register value. Reset command is not acceptable in
    ///Power Down mode.
    pub fn reset(&mut self) -> Result<(), Bh1750Error<I2C::Error>> {
        let data = 0b0000_0111;
        self.i2c
            .borrow_mut()
            .write(self.address, &[data])
            .map_err(Bh1750Error::I2cError)?;
        Ok(())
    }

    ///No active state.
    pub fn power_off(&mut self) -> Result<(), Bh1750Error<I2C::Error>> {
        let data = 0b0000_0000;
        self.i2c
            .borrow_mut()
            .write(self.address, &[data])
            .map_err(Bh1750Error::I2cError)?;
        Ok(())
    }
}
