//! This module contains a set of useful functions to operate on the prime numbers.
//!
//! Author: Denis Tsvikevich

use rand::Rng;
use crate::numbers::operations::mod_exp::{ModExp};

/// Generates a vector of prime numbers smaller than or equal to given number.
///
/// Note: This function uses the sieve of Eratosthenes.
pub fn generate(upto: usize) -> Vec<usize> {
    // Note that in this vector numbers are shifted i.e.
    // value with index 0 stands for one, and value with index 1 stands for two, etc.
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
    prime_flags.into_iter().enumerate().filter_map(|(index, is_prime)| if is_prime { Some(index + 1) } else { None }).collect()
}

/// Determine if a number is a prime.
///
/// Note: This function uses trial division.
pub fn is_prime_trial(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    let square_root = (n as f64).sqrt().trunc() as u64;
    return !(2..=square_root).any(|i| n % i == 0);
}

/// Determine if a number is a prime.
///
/// Note:
///     This function uses probabilistic method.
///     To make it more precise, you can enlarge repeats count.
///
/// Arguments:
///
/// * `n`: The number to test for primality.
/// * `repeats_count`: The number of times to repeat the test.
///
/// Returns:
///
/// When given number is prime - returns true, false otherwise.
pub fn fermat_primality_test(n: u64, repeats_count: u32) -> bool {
    if n < 2 {
        return false;
    }

    if n < 4 {
        return true;
    }

    let mut rng = rand::thread_rng();

    let mut i = 0;
    while i < repeats_count {
        let random_number = rng.gen_range(2..=(n - 2));
        if ModExp::mod_exp(random_number, n-1, n) != 1 {
            return false;
        }

        i += 1;
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_upto_thirty_should_be_generated_correctly() {
        let expected_prime_numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        let actual_generated_prime_numbers = generate(30);

        assert_eq!(actual_generated_prime_numbers, expected_prime_numbers);
    }

    #[test]
    fn all_is_prime() {
        let prime_numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        let is_every_number_in_vector_prime = prime_numbers.into_iter().all(|x| is_prime_trial(x));

        assert!(is_every_number_in_vector_prime);
    }

    #[test]
    fn all_is_not_prime() {
        let not_prime_numbers = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22];

        let is_every_number_in_vector_not_prime = not_prime_numbers.into_iter().all(|x| !is_prime_trial(x));

        assert!(is_every_number_in_vector_not_prime);
    }

    #[test]
    fn zero_is_not_prime() {
        let not_prime_number = 0;

        let is_prime_number = is_prime_trial(not_prime_number);

        assert_eq!(is_prime_number, false);
    }

    #[test]
    fn one_is_not_prime() {
        let not_prime_number = 1;

        let is_prime_number = is_prime_trial(not_prime_number);

        assert_eq!(is_prime_number, false);
    }

    #[test]
    fn zero_is_not_prime_fermat() {
        let not_prime_number = 0;

        let is_prime_number = fermat_primality_test(not_prime_number, 10);

        assert_eq!(is_prime_number, false, "Fermat primality test counted 0 as prime number");
    }

    #[test]
    fn all_is_prime_fermat() {
        let prime_numbers = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

        let is_every_number_in_vector_prime = prime_numbers.into_iter().all(|n| fermat_primality_test(n, 100));

        assert!(is_every_number_in_vector_prime, "Fermat primality test counted some number in the given vector as a composite");
    }

    #[test]
    fn all_is_not_prime_fermat() {
        let not_prime_numbers = vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22];

        let is_every_number_in_vector_not_prime = not_prime_numbers.into_iter().all(|n| !fermat_primality_test(n, 100));

        assert!(is_every_number_in_vector_not_prime, "Fermat primality test counted some number in the given vector as a prime");
    }
}