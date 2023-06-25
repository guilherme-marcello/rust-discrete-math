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

fn totient(n: u32) -> u32 {
    let primes = decompose_into_primes(n);
    println!("Prime factors of {}: {:?}", n, primes);
    totient_aux(primes)
}

fn totient_aux(mut prime_factors: Vec<u32>) -> u32 {
    match prime_factors.pop() {
        Some(head) => (head -  1) * totient_aux(prime_factors),
        None => 1,
    }
}

fn main() {
    let number = 4294967295;
    let ttt = totient(number);
    println!("Totient of {} is {}", number, ttt);
}