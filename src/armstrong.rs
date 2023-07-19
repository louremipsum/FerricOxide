pub fn is_armstrong_number(num: u32) -> bool {
    let mut n: i64 = num as i64;
    let mut sum: i64 = 0;
    let mut n_len: u32 = 0;
    while n > 0 {
        n_len += 1;
        n /= 10;
    }
    println!("Digits: {}", n_len);
    n = num as i64;
    for _i in 1..=n_len {
        let k: i64 = n % 10;
        sum += k.pow(n_len);
        println!("sum, {}", sum);
        n /= 10;
    }
    println!("final sum, {}", sum);
    if sum == num as i64 {
        println!("True");
        return true;
    }
    println!("False");
    false
}
