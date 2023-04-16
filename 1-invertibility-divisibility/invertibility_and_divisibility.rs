#[allow(dead_code)]
#[path = "../0-euclidian-algorithm/euclid.rs"] mod euclid;
use euclid::gcd;

/*
 * Returns true if (a,b) are coprime. false otherwise
 * @param `a`   - First integer
 * @param `b`   - Second integer
 */
fn coprime(a: i32, b: i32) -> bool {
    return gcd(a, b) == 1;
}

/*
 * Returns true if a is invertible in mod modulus. false otherwise
 * @param `a`       - Value of the modular integer
 * @param `modulus` - Modulus
 */
fn is_invertible(a: i32, modulus: i32) -> bool {
    return coprime(a, modulus);
}

fn main() {
    let mut i = 1;
    while i < 1000000 {
        assert_eq!(is_invertible(i, i + 1), true);
        i += 1;
    }
}