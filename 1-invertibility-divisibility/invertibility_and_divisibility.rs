#[allow(dead_code)]
#[path = "../0-euclidian-algorithm/euclid.rs"] mod euclid;
use euclid::gcd;
use euclid::ext;

/*
 * Returns true if (a,b) are coprime. false otherwise
 * @param `a`   - First integer
 * @param `b`   - Second integer
 */
pub fn coprime(a: i32, b: i32) -> bool {
    return gcd(a, b) == 1;
}

/*
 * Returns true if a is invertible in mod modulus. false otherwise
 * @param `a`       - Value of the modular integer
 * @param `modulus` - Modulus
 */
pub fn is_invertible(a: i32, modulus: i32) -> bool {
    return coprime(a, modulus);
}

/*
 * Returns the positive multiplicative inverse of a in Z_{modulus}.
 * @requires is_invertible(a, modulus)
 * @ensures \result > 0
 * @param `a`       - Value of the modular integer
 * @param `modulus` - Modulus
 */
pub fn inverse(a: i32, modulus: i32) -> i32 {
    let (x, _, _) = ext(a, modulus);
    return if x < 0 {x + modulus} else {x};
}

fn main() {
    let mut i = 1;
    while i < 1000000 {
        assert_eq!(is_invertible(i, i + 1), true);
        i += 1;
    }

    let a = 3;
    let n = 11;
    let a_inverse = inverse(a, n);
    let neg_a_inverse = a_inverse - n;

    println!("x.{a} ≡ 1 mod {n} <=>");
    println!("{a_inverse}.{a} ≡ 1 mod {n}");
    println!("{a_inverse} and {neg_a_inverse} are multiplicative inverses of {a} in Z_{n}");
}