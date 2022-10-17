use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);

    let result = a + b;

    println!("{} + {}i", result.re, result.im);

    // let a = 10;
    // let b: i32 = 10;
    // let c = 30i32;
    // let d = 30_i32;
    // let e = add(add(a, b), add(c, d));
    // println!("====================================");
    // println!("a + b + c + d = {}", e);
    //
    // // 2.3.1 Integers and decimal numbers
    // let twenty = 20;
    // let twenty_one: i32 = 21;
    // let twenty_two = 22i32;
    //
    // let addition = twenty + twenty_one + twenty_two;
    //
    // println!("====================================");
    // println!(
    //     "{} + {} + {} = {}",
    //     twenty, twenty_one, twenty_two, addition
    // );
    //
    // let one_million: i64 = 1_000_000;
    //
    // println!("====================================");
    // println!("{}", one_million.pow(2));
    //
    // let forty_twos = [42.0, 42f32, 42.0_f32];
    //
    // println!("====================================");
    //
    // println!("{:2}", forty_twos[0]);
    //
    // // 2.3.2 Integers with base 2, base 8, base 16
    //
    // let three = 0b11;
    // let thirty = 0o36;
    // let three_hundred = 0x12C;
    //
    // println!("base 10: {} {} {} ", three, thirty, three_hundred);
    // println!("base 2: {:b} {:b} {:b} ", three, thirty, three_hundred);
    // println!("base 8: {:o} {:o} {:o} ", three, thirty, three_hundred);
    // println!("base 16: {:x} {:x} {:x} ", three, thirty, three_hundred);
    //
    // println!("=========================================")
}
//
// fn add(a: i32, b: i32) -> i32 {
//     a + b
// }
