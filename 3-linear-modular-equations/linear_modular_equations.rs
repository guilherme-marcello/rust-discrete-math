#[allow(dead_code)]
#[path = "../2-modular-arithmetic/modular_arithmetic.rs"] mod modular_arithmetic;
#[path = "../1-invertibility-divisibility/invertibility_and_divisibility.rs"] mod invertibility_and_divisibility;
#[path = "../0-euclidian-algorithm/euclid.rs"] mod euclid;
use invertibility_and_divisibility::inverse;
use modular_arithmetic::ModInteger;

/*
 * Returns (x mod b) such that a.x ≡ b
 * @param `a`   - Coefficient of x
 * @param `b`   - Modular integer congruent to a.x
 */
fn solve(a: i32, b: &ModInteger) -> ModInteger {
    let a_inverse = inverse(a, b.modulus());
    b.mul(a_inverse)
}

fn main() {
    // given the linear equation: 4x ≡ 3 mod 11
    let a = 4;
    let b = ModInteger::new(3, 11);
    let x = solve(a, &b);
    println!("{}.x ≡ {} <=>", a, b.to_string());
    println!("x ≡ {}", x.to_string());   
}