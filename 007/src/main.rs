fn is_prime(n: i32) -> bool {
    let x = (n as f32).sqrt() as i32 + 1;
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
    let mut primes = 0;
    let mut i = 1;
    let mut largest = 0;
    while primes <= 10001 {
        if is_prime(i) {
            largest = i;
            primes += 1;
        }
        i += 1;
    }
    println!("{}", largest);
}
