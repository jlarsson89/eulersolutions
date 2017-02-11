fn main() {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut tot: u32 = 0;
    loop {
        let temp = a + b;
        a = b;
        b = temp;
        if a >= 10000000 { break; }
        if (a % 2 == 0) && (a < 4000000) {
            tot += a;
        }
    }
    println!("{}", tot);
}
