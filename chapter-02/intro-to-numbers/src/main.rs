fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32
    ];

    println!("{:5}", forty_twos[0]);
    println!("{:05}", forty_twos[0]);
    println!("{:.2}", forty_twos[0]);
    println!("{:.5}", forty_twos[0]);

    let three = 0b11; // 0b indicates binary (base 2) numerals
    let thirty = 0o36; // 0o indicates octal (base 8)
    let three_hundred = 0x12C; // 0x indicates hexadecimal (base 16)

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base  2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base  8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
