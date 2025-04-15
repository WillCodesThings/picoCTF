use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::collections::HashMap;
use std::env;
use std::str::FromStr;

/// GCD using Euclid’s algorithm
fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
    if b.is_zero() {
        a.clone()
    } else {
        gcd(b, &(a % b))
    }
}

/// LCM of two BigUints
fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    (a * b) / gcd(a, b)
}

/// Trial division for BigUint -> HashMap<base, exponent>
fn prime_factors(n: &BigUint) -> HashMap<BigUint, u32> {
    let mut n = n.clone();
    let mut d = BigUint::from(2u32);
    let mut factors = HashMap::new();

    while &d * &d <= n {
        let mut count = 0;
        while &n % &d == BigUint::zero() {
            n /= &d;
            count += 1;
        }
        if count > 0 {
            factors.insert(d.clone(), count);
        }
        d += BigUint::one();
    }

    if n > BigUint::one() {
        factors.insert(n, 1);
    }

    factors
}

/// λ(p^k) for BigUint
fn lambda_prime_power(p: &BigUint, k: u32) -> BigUint {
    if *p == BigUint::from(2u32) {
        match k {
            1 => BigUint::one(),
            2 => BigUint::from(2u32),
            _ => BigUint::from(1u32) << (k - 2), // 2^(k-2)
        }
    } else {
        (p - BigUint::one()) * p.pow(k - 1)
    }
}

/// Carmichael lambda function for BigUint
fn carmichael_lambda(n: &BigUint) -> BigUint {
    if *n == BigUint::one() {
        return BigUint::one();
    }

    let factors = prime_factors(n);
    factors
        .iter()
        .map(|(p, &k)| lambda_prime_power(p, k))
        .fold(BigUint::one(), |acc, x| lcm(&acc, &x))
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <large_integer>", args[0]);
        std::process::exit(1);
    }

    let n = match BigUint::from_str(&args[1]) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: '{}' is not a valid integer", args[1]);
            std::process::exit(1);
        }
    };

    let result = carmichael_lambda(&n);
    println!("λ({}) = {}", n, result);
}
// This program computes the Carmichael function λ(n) for a given positive integer n.
// It uses BigUint from the num-bigint crate to handle large integers.
// The program takes a single command-line argument, which is the integer n.
// It computes the prime factorization of n, calculates λ(p^k) for each prime factor p^k,
// and then computes the least common multiple (LCM) of these values to get λ(n).
// The program handles large integers efficiently and prints the result to the console.