#[allow(dead_code)]
#[path = "../2-modular-arithmetic/modular_arithmetic.rs"] mod modular_arithmetic;
#[path = "../1-invertibility-divisibility/invertibility_and_divisibility.rs"] mod invertibility_and_divisibility;
#[path = "../0-euclidian-algorithm/euclid.rs"] mod euclid;
use euclid::gcd;
use invertibility_and_divisibility::inverse;
use modular_arithmetic::ModInteger;

/*
 * Returns (x mod b) such that a.x ≡ b
 * @param `a`   - Coefficient of x
 * @param `b`   - Modular integer congruent to a.x
 */
fn solve(a: i32, b: &ModInteger) -> Vec<ModInteger> {
    println!("Equation is: {}.x ≡ {}", a, b.to_string());
    // d = (a, b.modulus())
    let d = gcd(a, b.modulus());

    // d | b.value() => c.d = b.value()
    let c = b.value() / d;
    assert!(c*d == b.value(), "There are no solutions: (a, m) ∤ b ");

    // reduced equation: (a/d)x ≡ (b.value()/d) mod (b.modulus()/d)
    let reduced_b = ModInteger::new(
        b.value() / d, b.modulus() / d
    );
    
    let a_inverse = inverse(a / d, reduced_b.modulus());
    let x0 = reduced_b.mul(a_inverse);

    println!("Reduced equation is: {}.x ≡ {} <=> x ≡ {}", a / d, reduced_b.to_string(), x0.to_string());


    // find all solutions
    let mut solutions = Vec::new();
    for k in 0..d {
        solutions.push(
            ModInteger::new(
                x0.value() + reduced_b.modulus()*k, b.modulus()
            )
        );
    }
    solutions
}

fn main() {
    // given the linear equation: 35x ≡ 10 mod 50
    let a = 35;
    let b = ModInteger::new(10, 50);
    let solutions = solve(a, &b);
    println!("Solutions:");
    for solution in &solutions {
        println!("x ≡ {}", solution.to_string()); 
    }
      
}