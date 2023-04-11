/*
 * Returns (a, b), using Euclidian Algorithm
 * @param `a`   - First integer
 * @param `b`   - Second integer
 */
fn gcd(a: i32, b: i32) -> i32 {
    let r = a % b; 
    if r == 0 {
        return b;
    }
    return gcd(b, r);
}

fn main() {
    let a = 7;
    let b = 3;
    println!("({}, {}) = {}", a, b, gcd(a, b))
}