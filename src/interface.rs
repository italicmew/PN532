pub trait WriteFrame {
    type Error;
    fn write_frame(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

pub enum PN532Error<InterfaceError> {
    /// I²C / SPI Error
    IOError(InterfaceError),
}