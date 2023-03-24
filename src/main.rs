
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
    const VITERBI_POLY_G1:u8 = 0x4f; // 0100 1111
    const VITERBI_POLY_G2:u8 = 0x6d; // 0110 1101

    let mut resister:u8 = 0;
    for (i, byte) in cadu.iter().enumerate() {
        for bit in 0..8 {
            resister = (resister << 1) | ((byte >> (7-bit)) & 1);
            symbols[2*(i*8+bit)+0] = parity(resister & VITERBI_POLY_G1)*255; // generally, symbols is a voltage of demodulator, so we multiply 255 here.
            symbols[2*(i*8+bit)+1] = parity(resister & VITERBI_POLY_G2)*255;
        }
    }
    for bit in 0..6 { // flush resister
        resister = resister << 1;
        symbols[2*(CCSDS_CADU_LENGTH*8+bit)+0] = parity(resister & VITERBI_POLY_G1)*255;
        symbols[2*(CCSDS_CADU_LENGTH*8+bit)+1] = parity(resister & VITERBI_POLY_G2)*255;
    }

    // println!("{:?}", symbols);

    let mut symbols_table: [[u8; 32]; 2] = [[0; 32]; 2];
    // symbols_table is a result symbol which from the state of the final register
    //   e.g. symbol becomes symbols_table[x][0] if final resiter is 000000. This means that original resiter was [0/1]00000 and bit=0 was input
    //   e.g. symbol becomes symbols_table[x][1] if final resiter is 000010. This means that original resiter was [0/1]00001 and bit=0 was input
    // (255 - symbols_table[x][i]) can be used as a complement, so we can complete all 64 states of 6 bit resister
    //   e.g. symbol becomes (255 - symbols_table[x][0]) if final resiter is 111111. This means that original resiter was [0/1]11111 then bit=1 was input
    //   e.g. symbol becomes (255 - symbols_table[x][1]) if final resiter is 111101. This means that original resiter was [0/1]11110 then bit=1 was input
    for i in 0..32 {
        symbols_table[0][i] = parity((i as u8)*2 & VITERBI_POLY_G1)*255;
        symbols_table[1][i] = parity((i as u8)*2 & VITERBI_POLY_G2)*255;
    }
    // println!("{:?}", symbols_table);
}
