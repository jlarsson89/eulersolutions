fn main() {
    let mut a: i32 = 0;
    let mut b: i32 = 1;
    let mut total: i32 = 0;
    loop {
        let temp = a + b;
        a = b;
        b = temp;
        if a >= 10000000 { 
        	break;
        }
        if (a % 2 == 0) && (a < 4000000) {
            total += a;
        }
    }
    println!("{}", total);
}
