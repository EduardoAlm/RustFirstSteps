fn main() {
    println!("Hello, world!");
    println!("GCD: {}", greatest_common_divisor(12, 23));
}


fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m!= 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(greatest_common_divisor(14, 15), 1);
    assert_eq!(greatest_common_divisor(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}