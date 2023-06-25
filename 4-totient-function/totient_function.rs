fn decompose_into_primes(n: u32) -> Vec<u32> {
    let mut prime_factors = Vec::new();
    let mut num = n;
    let mut divisor = 2; // smallest prime -> 2

    while num > 1 {
        if num % divisor == 0 {
            // 'divisor' is a prime factor
            prime_factors.push(divisor);
            // reduce num
            num /= divisor;
        } else {
            divisor += 1; // !(divisor | num) => next divisor
        }
    }

    prime_factors
}