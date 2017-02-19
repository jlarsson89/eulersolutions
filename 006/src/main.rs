fn sum_of_squares(n: i32) -> i32 {
    let mut total = 0;
    for i in 1..n+1 {
        total += i * i;
    }
    return total;
}

fn square_of_sums(n: i32) -> i32 {
    let mut numbers = 0;
    for i in 1..n+1 {
        numbers += i;
    }
    return numbers * numbers;
}

fn main() {
    println!("{}", square_of_sums(100) - sum_of_squares(100));
}
