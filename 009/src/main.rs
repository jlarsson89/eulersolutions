use std::process;

fn main() {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if a * a + b * b == c * c && a + b + c == 1000 {
                    println!("{}", a * b * c);
                    process::exit(1);
                }
            }
        }
    }
}
