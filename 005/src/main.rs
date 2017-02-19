fn is_prime(n: i32) -> bool {
    if n < 2 { 
        return false;
    }
    if n < 4 { 
        return true;
    }
    for i in 2..n {
        if n % i == 0 { 
            return false;
        }
    }
    true
}
fn add_prime(mut primes: Vec<i32>, n: i32) -> Vec<i32>  {
    if is_prime(n) {
        primes.push(n);
    }
    return primes;
}
fn main() {
    let mut primes: Vec<i32> = Vec::new();
    let mut total = 1;
    let mut x = 1;
    while x < 21 {
        primes = add_prime(primes, x);
        x += 1;
    }
    for i in primes {
        let mut j = i;
        while j < 20 {
            j = j * i;
        }
        j = j / i;
        total = total * j;
    }
    println!("{}", total);
}
