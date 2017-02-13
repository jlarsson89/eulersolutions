fn main() {
    let mut palindromes: Vec<i32> = Vec::new();
    let mut largest: i32 = 0;
    for i in 1..999 {
        for j in 1..999 {
            let k: String = (i * j).to_string();
            let l: String = k.chars().rev().collect::<String>();
            if k == l { 
                let x: i32 = k.parse().unwrap();
                palindromes.push(x); 
            }
        }
    }
    for i in palindromes {
        let x: i32 = i;
        if x > largest {
            largest = x;
        }
    }
    println!("{}", largest);
}
