use core::fmt;

#[derive(Debug)]
pub enum Bh1750Error<E: fmt::Debug> {
    I2cError(E),
    DeviceNotFoundError
}

impl<E: fmt::Debug> fmt::Display for Bh1750Error<E> {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        match self {
            Bh1750Error::I2cError(e) => write!(f, "I2C error: {:?}", e),
            Bh1750Error::DeviceNotFoundError => write!(f, "No device found!")
        }
    }

}