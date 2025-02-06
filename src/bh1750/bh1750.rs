use embedded_hal::i2c::I2c;
use esp_hal::i2c::master::Error;
use crate::bh1750::Command;

pub struct Bh1750<'a> {
    pub i2c: &'a mut dyn I2c<Error = Error>,
    pub address: u8,
}

impl<'a> Bh1750<'a> {
    pub fn new(i2c: &'a mut dyn I2c<Error = Error>, address: u8) -> Self {
        Self { i2c, address }
    }

    pub fn read(&mut self, command: Command) -> Result<u16, Error> {
        let mut result = [0u8; 2];
        self.i2c.write_read(self.address, &[command as u8], &mut result)?;
        Ok(u16::from_be_bytes(result))
    }

    ///Waiting for measurement command.
    pub fn power_on(&mut self) -> Result<(), Error> {
        let data = 0b0000_0001;
        self.i2c.write(self.address, &[data])
    }
    ///Reset Data register value. Reset command is not acceptable in
    ///Power Down mode.
    pub fn reset(&mut self) -> Result<(), Error> {
        let data = 0b0000_001;
        self.i2c.write(self.address, &[data])
    }

    ///No active state.
    pub fn power_off(&mut self) -> Result<(), Error> {
        let data = 0b0000_0000;
        self.i2c.write(self.address, &[data])
    }

}
