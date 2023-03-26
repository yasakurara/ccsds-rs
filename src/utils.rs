/// return 1 if total number of 1 in byte is odd number.
/// we should create a parity table to have good performance.
pub fn parity(byte: u8) -> u8 {
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