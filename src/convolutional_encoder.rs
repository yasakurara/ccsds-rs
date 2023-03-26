use crate::ccsds;
use crate::utils;


// Convolutional coding
// G1 = 1111001 and G2 = 1011011 are defined in CCSDS 131.0-B-4 "TM SYNCHRONIZATION AND CHANNEL CODING RECOMMENDED STANDARD".
// In "Basic Convolutional Encoder Block Diagram", the left bit of the resister is LSB. So, we need to turn over the bits of them.

pub const POLY_G1:u8 = 0x4f; // 0100 1111
pub const POLY_G2:u8 = 0x6d; // 0110 1101

#[derive(Debug)]
pub struct ConvolutionalEncoder {
    symbols: [u8; (8*ccsds::CCSDS_CADU_LENGTH+6)*2]
}

impl ConvolutionalEncoder {
    pub fn new() -> ConvolutionalEncoder {
        let symbols = [0; (8*ccsds::CCSDS_CADU_LENGTH+6)*2];
        ConvolutionalEncoder {
            symbols
        }
    }

    pub fn encode(&mut self, bytes: &[u8]) {
        let mut resister:u8 = 0;
        for (i, byte) in bytes.iter().enumerate() {
            for bit in 0..8 {
                resister = (resister << 1) | ((byte >> (7-bit)) & 1);
                self.symbols[2*(i*8+bit)+0] = utils::parity(resister & POLY_G1)*255; // generally, symbols is a voltage of demodulator, so we multiply 255 here.
                self.symbols[2*(i*8+bit)+1] = utils::parity(resister & POLY_G2)*255;
            }
        }
        for bit in 0..6 { // flush resister
            resister = resister << 1;
            self.symbols[2*(ccsds::CCSDS_CADU_LENGTH*8+bit)+0] = utils::parity(resister & POLY_G1)*255;
            self.symbols[2*(ccsds::CCSDS_CADU_LENGTH*8+bit)+1] = utils::parity(resister & POLY_G2)*255;
        }
    }

    pub fn get_symbols(&self) -> &[u8] {
        &self.symbols[..]
    }
}