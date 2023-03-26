mod ccsds;
mod convolutional_encoder;
mod utils;

fn main() {

    let mut cadu: [u8; ccsds::CCSDS_CADU_LENGTH] = [0; ccsds::CCSDS_CADU_LENGTH];

    // Set ASM BIT PATTERNS
    cadu[0] = 0x1a;
    cadu[1] = 0xcf;
    cadu[2] = 0xfc;
    cadu[3] = 0x1d;

    let mut _ce = convolutional_encoder::ConvolutionalEncoder::new();
    _ce.encode(&cadu);
    // println!("{:?}",_ce.get_symbols());

    // Viterbi decoding

    let mut symbols_table: [[u8; 32]; 2] = [[0; 32]; 2];
    // symbols_table is a result symbol which from the state of the final register
    //   e.g. symbol becomes symbols_table[x][0] if final resiter is 000000. This means that original resiter was [0/1]00000 and bit=0 was input
    //   e.g. symbol becomes symbols_table[x][1] if final resiter is 000010. This means that original resiter was [0/1]00001 and bit=0 was input
    // (255 - symbols_table[x][i]) can be used as a complement, so we can complete all 64 states of 6 bit resister
    //   e.g. symbol becomes (255 - symbols_table[x][0]) if final resiter is 111111. This means that original resiter was [0/1]11111 then bit=1 was input
    //   e.g. symbol becomes (255 - symbols_table[x][1]) if final resiter is 111101. This means that original resiter was [0/1]11110 then bit=1 was input
    for i in 0..32 {
        symbols_table[0][i] = utils::parity((i as u8)*2 & convolutional_encoder::POLY_G1)*255;
        symbols_table[1][i] = utils::parity((i as u8)*2 & convolutional_encoder::POLY_G2)*255;
    }
    // println!("{:?}", symbols_table);
}
