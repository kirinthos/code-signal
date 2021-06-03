fn magicalWell(a: i32, b: i32, n: i32) -> i32 {
    (0..n).fold(0, |acc, n| acc + (a + n) * (b + n))
}

fn main() {
    let a = 1;
    let b = 2;
    let n = 2;
    println!("({}, {}) = {:?}", a, b, magicalWell(a, b, n));
}
