# 0: Euclidean Algorithm

The Euclidean algorithm is a simple and efficient method for finding the greatest common divisor (GCD) of two integers. It is named after the ancient Greek mathematician Euclid.

## How it Works

The Euclidean algorithm takes two positive integers, `a` and `b`, as input and computes their GCD through successive divisions by the remainder. It continues this process until the remainder becomes equal to zero. At this point, the GCD is equal to the last remaining non-zero remainder obtained before reaching zero.

Here's an Haskell snippet representation of the Euclidean algorithm:
```haskell
gcd :: Int -> Int -> Int
gcd a b
  | r == 0 = b
  | otherwise = gcd b r
  where
    r = mod a b
```
In the pseudocode, `a` and `b` are the input integers, and `mod` denotes the modulo operation, which gives the remainder `r` when `a` is divided by `b`. The algorithm continues to update the values of `a` and `b` by assigning `b` to `a` and `r` to `b`, until `r` becomes zero. At this point, `b` holds the GCD of the original input integers.

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

The Euclidean algorithm also has an extended version called the Extended Euclidean algorithm, which computes the coefficients of Bézout's identity. Bézout's identity states that for any two integers `a` and `b`, there exist integers `x` and `y` such that `ax + by = gcd(a, b)`. The Extended Euclidean algorithm can be used to find these coefficients, which can be helpful in solving linear Diophantine equations.

We establish a relationship among the lines $i_{+ 1}$, $i$, and $i_{- 1}$ based on the Extended Euclidean Algorithm in the following manner:
$$ d_2 = d_0 - q_1d_1 $$
$$ x_2 = x_0 - q_1x_1 $$
$$ y_2 = y_0 - q_1y_1 $$
And subsequently, this relationship can be computed and tabulated to obtain the resulting values of x, y, and gcd(a, b):
| $i$ |       $a_i$       |     $q_i$     |         $x_i$         |         $y_i$        |
|:---:|:-----------------:|:-------------:|:---------------------:|:--------------------:|
|  0  |        $a$        |               |           1           |           0          |
|  1  |        $b$        | $\frac{a}{b}$ |           0           |           1          |
|  2  |  $a - q_1\cdot b$ |       ..      |  $x_0 - q_1\cdot x_1$ | $y_0 - q_1\cdot y_1$ |


Here's a python snippet representation of the Extended Euclidean algorithm:
```python
def ext(d0, d1, q1, x0 = 1, x1 = 0, y0 = 0, y1 = 1) -> (int, int, int):
    # compute next line as the remainder of d0 // d2
    d2 = d0 - q1*d1
    if d2 == 0:
        # if remainder is zero, return line above
        return (x1, y1, d1)
    # compute next line, from the two lines above
    x2 = x0 - q1*x1
    y2 = y0 - q1*y1
    # 
    return ext (d1, d2, d1 // d2, x1, x2, y1, y2)
```
This recursive process continues until `d2` (remainder of `d0 / d1`) becomes zero, at which point the function returns the final values of `x1`, `y1`, and `d1`, which represent the coefficients of Bézout's identity and the GCD of the original input integers, respectively.