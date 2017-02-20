fn main() {
    let mut array: Vec<i64> = Vec::new();
    let mut i: i64 = 2;
    let mut n: i64 = 600851475143;
    let mut largest: i64 = 0;
    while (i * i) < n {
        while n % i == 0 {
            array.push(i);
            n /= i;
        }
        i += 1;
    }
    if n > 1 {
    	array.push(n) 
    }
    loop {
        let top = match array.pop() {
            None => break,
            Some(x) => x,
        };
        if top > largest { 
        	largest = top 
        }
    }
    println!("{}", largest);
}
