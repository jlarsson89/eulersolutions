fn is_prime(n: i64) -> bool {
    let x = (n as f64).sqrt() as i64 + 1;
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    for i in 2..x {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut i: i64 = 2;
    let mut total: i64 = 0;
    let end: i64 = 2000000;
    while i <= end {
        if is_prime(i) {
            total += i;
        }
        if i % 2 == 0 {
            i += 1;
        }
        else {
            i += 2;
        }
    }
    println!("{}", total);
}
