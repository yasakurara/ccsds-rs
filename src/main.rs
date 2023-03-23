
/// return 1 if total number of 1 in byte is odd number.
/// we should create a parity table to have good performance.
fn parity(byte: u8) -> u8 {
    let mut a: u8 = 1;
    a = a ^ (byte);
    a = a ^ (byte >> 1);
    a = a ^ (byte >> 2);
    a = a ^ (byte >> 3);
    a = a ^ (byte >> 4);
    a = a ^ (byte >> 5);
    a = a ^ (byte >> 6);
    a = a ^ (byte >> 7);
    !a & 1
}

fn main() {
    const CCSDS_CADU_LENGTH: usize = 256;

    let mut cadu: [u8; CCSDS_CADU_LENGTH] = [0; CCSDS_CADU_LENGTH];

    // Set ASM BIT PATTERNS
    cadu[0] = 0x1a;
    cadu[1] = 0xcf;
    cadu[2] = 0xfc;
    cadu[3] = 0x1d;

    let mut symbols: [u8; (8*CCSDS_CADU_LENGTH+6)*2] = [0; (8*CCSDS_CADU_LENGTH+6)*2];

    // Convolutional coding
    // G1 = 1111001 and G2 = 1011011 are defined in CCSDS 131.0-B-4 "TM SYNCHRONIZATION AND CHANNEL CODING RECOMMENDED STANDARD".
    // In "Basic Convolutional Encoder Block Diagram", the left bit of the resister is LSB. So, we need to turn over the bits of them.
    const VITERBI_POLYG1:u8 = 0x4f; // 0100 1111
    const VITERBI_POLYG2:u8 = 0x6d; // 0110 1101

    let mut resister:u8 = 0;
    for (i, byte) in cadu.iter().enumerate() {
        for bit in 0..8 {
            resister = (resister << 1) | ((byte >> (7-bit)) & 1);
            symbols[2*(i*8+bit)+0] = parity(resister & VITERBI_POLYG1);
            symbols[2*(i*8+bit)+1] = parity(resister & VITERBI_POLYG2);
        }
    }
    for bit in 0..6 { // flush resister
        resister = resister << 1;
        symbols[2*(CCSDS_CADU_LENGTH*8+bit)+0] = parity(resister & VITERBI_POLYG1);
        symbols[2*(CCSDS_CADU_LENGTH*8+bit)+1] = parity(resister & VITERBI_POLYG2);
    }

    println!("{:?}", symbols);
}
