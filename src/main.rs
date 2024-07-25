use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

fn euler_phi(mut n: BigUint) -> BigUint {
    if n.is_one() {
        return BigUint::one();
    }

    let mut result = n.clone();
    let mut p = BigUint::from(2u32);

    while &p * &p <= n {
        if &n % &p == BigUint::zero() {
            while &n % &p == BigUint::zero() {
                n /= &p;
            }
            result -= &result / &p;
        }
        p += BigUint::one();
    }

    if n > BigUint::one() {
        result -= &result / &n;
    }

    result
}

fn main() {
    let n = BigUint::from_str("4900555555555563333").unwrap();
    println!("Euler's Totient Function phi({}) = {}", n, euler_phi(n.clone()));
}



