fn main() {
    let numbers = "73167176531330624919225119674426574742355349194934
96983520312774506326239578318016984801869478851843
85861560789112949495459501737958331952853208805511
12540698747158523863050715693290963295227443043557
66896648950445244523161731856403098711121722383113
62229893423380308135336276614282806444486645238749
30358907296290491560440772390713810515859307960866
70172427121883998797908792274921901699720888093776
65727333001053367881220235421809751254540594752243
52584907711670556013604839586446706324415722155397
53697817977846174064955149290862569321978468622482
83972241375657056057490261407972968652414535100474
82166370484403199890008895243450658541227588666881
16427171479924442928230863465674813919123162824586
17866458359124566529476545682848912883142607690042
24219022671055626321111109370544217506941658960408
07198403850962455444362981230987879927244284909188
84580156166097919133875499200524063689912560717606
05886116467109405077541002256983155200055935729725
71636269561882670428252483600823257530420752963450";
    let mut start = 0;
    let mut end = 13;
    let mut largest = 0;
    let mut sum = 0;
    let numbers = numbers.replace("\n", "");
    let sequence: Vec<char> = numbers.chars().collect();
    for i in sequence.iter() {
        if (end == 987) {
            break;
        }
        let v1: String = sequence[start].to_string();
        let v1 = v1.parse::<i64>().unwrap();
        let v2: String = sequence[start+1].to_string();
        let v2 = v2.parse::<i64>().unwrap();
        let v3: String = sequence[start+2].to_string();
        let v3 = v3.parse::<i64>().unwrap();
        let v4: String = sequence[start+3].to_string();
        let v4 = v4.parse::<i64>().unwrap();
        let v5: String = sequence[start+4].to_string();
        let v5 = v5.parse::<i64>().unwrap();
        let v6: String = sequence[start+5].to_string();
        let v6 = v6.parse::<i64>().unwrap();
        let v7: String = sequence[start+6].to_string();
        let v7 = v7.parse::<i64>().unwrap();
        let v8: String = sequence[start+7].to_string();
        let v8 = v8.parse::<i64>().unwrap();
        let v9: String = sequence[start+8].to_string();
        let v9 = v9.parse::<i64>().unwrap();
        let v10: String = sequence[start+9].to_string();
        let v10 = v10.parse::<i64>().unwrap();
        let v11: String = sequence[start+10].to_string();
        let v11 = v11.parse::<i64>().unwrap();
        let v12: String = sequence[start+11].to_string();
        let v12 = v12.parse::<i64>().unwrap();
        let v13: String = sequence[start+12].to_string();
        let v13 = v13.parse::<i64>().unwrap();
        sum = v1 * v2 * v3 * v4 * v5 * v6 * v7 * v8 * v9 * v10 * v11 * v12 * v13;
        if sum > largest {
            largest = sum;
        }
        start += 1;
        end += 1;
    }
    println!("{}", largest);
}