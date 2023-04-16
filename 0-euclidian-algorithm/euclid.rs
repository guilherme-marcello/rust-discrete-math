/*
 * Returns (a, b), using Euclidian Algorithm
 * @param `a`   - First integer
 * @param `b`   - Second integer
 */
pub fn gcd(a: i32, b: i32) -> i32 {
    let r = a % b; 
    if r == 0 {
        return b;
    }
    return gcd(b, r);
}

/*
 * Returns a tuple with (x, y, d), using Euclidian Algorithm
 * @param `a`   - First integer
 * @param `b`   - Second integer
 */
fn ext(a: i32, b: i32) -> (i32, i32, i32) {
    return 
    if a > b {aux_ext(a, b, a / b, 1, 0, 0, 1)} else {aux_ext(a, b, a / b, 0, 1, 1, 0)};
}
/*
 * aux function for extended euclidian algorithm recursive calls
 */
fn aux_ext(d0: i32, d1: i32, q1: i32, x0: i32, x1: i32, y0: i32, y1: i32) -> (i32, i32, i32) {
    let d2 = d0 - q1*d1;
    if d2 == 0 {
        return (x1, y1, d1)
    }
    let x2 = x0 - q1*x1;
    let y2 = y0 - q1*y1;
    
    return aux_ext(d1, d2, d1 / d2, x1, x2, y1, y2);
}



fn main() {
    let a = 711;
    let b = 132;
    let d = gcd(a, b);
    println!("gcd({a}, {b}) = {d}");
    
    // extended version
    let (x, y, d_) = ext(a, b);
    assert_eq!(d, d_);
    println!("{a}.x + {b}.y = gcd({a},{b}) <=>");
    println!("{a}.x + {b}.y = {d_} <=>");
    println!("{a}.{x} + {b}.{y} = {d_}");
}
