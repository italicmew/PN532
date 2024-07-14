#![no_std]

use interface::{PN532Error, ReadFrame, WriteFrame};

mod i2c;
mod interface;

pub struct PN532<T> {
    interface: T,
}

impl<T, InterfaceError> PN532 <T>
where T: WriteFrame<Error = PN532Error<InterfaceError>>
        + ReadFrame<Error = PN532Error<InterfaceError>>
{
    // Communication between the host controller and the PN532 is performed through frames, in a half-duplex mode.
    // Four different types of frames are used in one or both directions (host controller to the 
    // PN532 and PN532 to the host controller).

    pub fn send_command(self, command: u8, params: &[u8]) -> Result<(), PN532Error<InterfaceError>> {
        let mut buff = [0u8; 255];
        buff[0] = 0xD4;
        buff[1] = command;

        for (i, &byte) in params.iter().enumerate() {
            buff[2 + i] = byte;
        }

        self.create_frame(&buff)
    }


    pub fn create_frame(mut self, data: &[u8]) -> Result<(), PN532Error<InterfaceError>> {
        let mut frame = [0u8; 255 + 7];
        frame[0] = 0x00; // Preamble
        frame[1] = 0x00; // Start Code 1
        frame[2] = 0xFF; // Start Code 2
        frame[3] = data.len() as u8; // Length
        frame[4] = (!data.len() as u8).wrapping_add(1); // Length Checksum (LCS)
    
        // Initialize checksum with TFI value (0xD4 for PN532)
        let mut checksum: u8 = 0xD4;
    
        // Copy data into frame and update checksum
        for (i, &byte) in data.iter().enumerate() {
            frame[5 + i] = byte;
            checksum = checksum.wrapping_add(byte);
        }
    
        // Calculate the Data Checksum (DCS)
        frame[data.len() + 5] = !checksum;
        frame[data.len() + 6] = 0x00; // Postamble

        self.interface.write_frame(&frame)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    pub struct NoOpInterface;
    #[derive(Debug)]
    pub struct NoOpError;

    impl WriteFrame for NoOpInterface {
        type Error = PN532Error<NoOpError>;
        fn write_frame(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    pub fn get_device() -> PN532<NoOpInterface> {
        PN532 {
            interface: NoOpInterface,
        }
    }

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
