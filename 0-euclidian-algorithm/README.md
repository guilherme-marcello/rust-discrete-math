# 0: Euclidean Algorithm

The Euclidean algorithm is a simple and efficient method for finding the greatest common divisor (GCD) of two integers. It is named after the ancient Greek mathematician Euclid.

## How it Works

The Euclidean algorithm takes two positive integers, `a` and `b`, as input and computes their GCD through successive divisions by the remainder. It continues this process until the remainder becomes equal to zero. At this point, the GCD is equal to the last remaining non-zero remainder obtained before reaching zero.

Here's an Haskell snippet representation of the Euclidean algorithm:
```haskell
gdc :: Int -> Int -> Int
gdc a b
  | r == 0 = b
  | otherwise = gdc b r
  where
    r = mod a b
```
In the pseudocode, `a` and `b` are the input integers, and `mod` denotes the modulo operation, which gives the remainder `r` when `a` is divided by `b`. The algorithm continues to update the values of `a` and `b` by assigning `a` to `r` and `a` to `b`, until `r` becomes zero. At this point, `b` holds the GCD of the original input integers.

In Rust, we can use recursion to implement the Euclidean algorithm and compute the GCD of two given integers `a` and `b`. Here's an example implementation in Rust:
```rust
fn gcd(a: i32, b: i32) -> i32 {
    let r = a % b; 
    if r == 0 {
        return b;
    }
    return gcd(b, r);
}
```