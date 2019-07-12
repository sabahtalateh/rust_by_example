fn main() {
    println!("1 + 2 = {}", 1u32 + 2);

    println!("1011 AND 1010 = {:04b}", 0b1011u32 & 0b1010u32);
    println!("1011 OR 1010 = {:04b}", 0b1011u32 | 0b1010u32);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {0} [0x{0:08b}]", 1u32 << 5);
    println!("0x{0:x} {0:08b} {0}", 0x80u32 >> 0);
}
