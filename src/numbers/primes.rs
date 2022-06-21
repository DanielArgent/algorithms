//! This module contains a set of useful functions to operate on the prime numbers.
//!
//! Author: Denis Tsvikevich

/// Generates a vector of prime numbers smaller than or equal to given number.
///
/// Note: This function uses the sieve of Eratosthenes.
pub fn generate(upto: usize) -> Vec<usize> {
    let mut prime_flags = vec![true; upto];

    // One is not a prime number.
    prime_flags[0] = false;

    let mut current_smallest_prime = 2usize;
    while current_smallest_prime <= upto {
        if prime_flags[current_smallest_prime - 1] {
            let mut current_multiplier = 2;

            // Mark every number that can be divided by the current smallest prime as a composite.
            loop {
                let multiple = current_smallest_prime * current_multiplier;

                if multiple > upto {
                    break;
                }

                prime_flags[(current_smallest_prime * current_multiplier) - 1] = false;
                current_multiplier += 1;
            }
        }

        current_smallest_prime += 1;
    }

    // Transform a vector of logical flags into a vector of prime numbers.
    prime_flags.into_iter().enumerate().filter(|(_, v)| { *v }).map(|(i,_)| { i + 1 }).collect()
}

/// Determine if a number is a prime.
///
/// Note: This function uses trial division.
pub fn is_prime_trial(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let square_root = (n as f64).sqrt().trunc() as u64;
    for i in 2..=square_root {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_upto_thirty_should_be_generated_correctly() {
        let primes_upto_thirty = generate(30);
        assert_eq!(primes_upto_thirty, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn all_is_prime() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        assert!(primes.into_iter().all(|x| is_prime_trial(x)));
    }

    #[test]
    fn all_is_not_prime() {
        let primes = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22];
        assert!(primes.into_iter().all(|x| !is_prime_trial(x)));
    }

    #[test]
    fn zero_is_not_prime() {
        let zero_is_prime = is_prime_trial(0);
        assert_eq!(zero_is_prime, false);
    }


    #[test]
    fn one_is_not_prime() {
        let zero_is_prime = is_prime_trial(1);
        assert_eq!(zero_is_prime, false);
    }
}