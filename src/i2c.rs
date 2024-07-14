use crate::{interface::{ReadFrame, WriteFrame}, PN532Error};

const PN532_I2C_ADDRESS: u8 = 0x48;

pub struct I2cInterface<I2C> {
    i2c: I2C,
}

impl<I2C, E> WriteFrame for I2cInterface<I2C>
where 
    I2C: embedded_hal::i2c::I2c<Error = E>
{
    type Error = PN532Error<E>;

    fn write_frame(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.i2c.write(PN532_I2C_ADDRESS, data).map_err(PN532Error::IOError)
    }
    
}

impl<I2C, E> ReadFrame for I2cInterface<I2C>
where 
    I2C: embedded_hal::i2c::I2c<Error = E>
{
    type Error = PN532Error<E>;

    fn read_frame(&mut self,  buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.i2c.read(PN532_I2C_ADDRESS, buffer).map_err(PN532Error::IOError)
    }
    
}