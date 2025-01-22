use embedded_hal::i2c::I2c;
use esp_hal::i2c::master::Error;

pub struct BH1750<'a> {
    pub i2c: &'a mut dyn I2c<Error = Error>,
    pub address: u8,
}

impl<'a> BH1750<'a> {
    pub fn new(i2c: &'a mut dyn I2c<Error = Error>, address: u8) -> Self {
        Self { i2c, address }
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

    ///Start measurement at 1lx resolution.
    ///Measurement Time is typically 120ms.
    ///It is automatically set to Power Down mode after measurement.
    pub fn one_time_h_resolution_mode(&mut self) -> Result<u16, Error> {
        let data = 0b0001_0000;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }

    ///Start measurement at 0.5lx resolution.
    ///Measurement Time is typically 120ms.
    ///It is automatically set to Power Down mode after measurement.
    pub fn one_time_h_resolution_mode2(self) -> Result<u16, Error> {
        let data = 0b0010_0001;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }

    ///Start measurement at 4lx resolution.
    ///Measurement Time is typically 16ms.
    ///It is automatically set to Power Down mode after measurement.
    pub fn one_time_l_resolution_mode(self) -> Result<u16, Error> {
        let data = 0b0010_0011;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }

    ///Start measurement at 1lx resolution.
    ///Measurement Time is typically 120ms.
    pub fn continuously_h_resolution_mode(self) -> Result<u16, Error> {
        let data = 0b0001_0000;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }
    ///Start measurement at 0.5lx resolution.
    ///Measurement Time is typically 120ms.
    pub fn continuously_h_resolution_mode2(self) -> Result<u16, Error> {
        let data = 0b0001_0001;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }
    ///Start measurement at 4lx resolution.
    ///Measurement Time is typically 16ms.
    pub fn continuously_l_resolution_mode(self) -> Result<u16, Error> {
        let data = 0b0001_0011;
        let mut response = [0u8; 2];
        self.i2c
            .write_read(self.address, &[data], &mut response)
            .unwrap();
        Ok(u16::from_be_bytes(response))
    }
    ///function checks all available addresses
    pub fn check_addresses(&mut self) {
        for address in 0x03..0x78 {
            if self.i2c.write(address, &[]).is_ok() {
                esp_println::println!("I2C device found at address: 0x{:02X}", address);
            }
        }
    }
}
