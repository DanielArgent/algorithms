//! This module contains a trait and implementations for evaluating modular exponentiation.
//!
//! Author: Denis Tsvikevich

use num::{Num};

/// Trait for numeric types that supports modular exponentiation.
pub trait ModExp: Num + PartialOrd + Copy {
    /// Performs modular exponentiation.
    ///
    /// Arguments:
    ///
    /// * `base`: The base of the exponentiation.
    /// * `exponent`: The exponent to raise the base to.
    /// * `modulus`: The modulus to use.
    ///
    /// Returns:
    ///
    /// The result of the modular exponentiation.
    fn mod_exp(base: Self, exponent: Self, modulus: Self) -> Self {
        if modulus == Self::one() {
            return Self::zero();
        }

        let mut result = Self::one();

        let mut i = Self::zero();
        while i < exponent {
            result = (result * base) % modulus;
            i = i + Self::one();
        }

        return result
    }
}

impl ModExp for i8 {}
impl ModExp for i16 {}
impl ModExp for i32 {}
impl ModExp for i64 {}
impl ModExp for i128 {}

impl ModExp for u8 {}
impl ModExp for u16 {}
impl ModExp for u32 {}
impl ModExp for u64 {}
impl ModExp for u128 {}

impl ModExp for usize {}
impl ModExp for isize {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn modular_exponentiation_of_4_13_and_497_should_be_equal_445() {
        let base = 4;
        let exponent = 13;
        let modulus = 497;

        let ans: i32 = ModExp::mod_exp(base,exponent, modulus);

        assert_eq!(ans, 445);
    }

    #[test]
    fn modular_exponentiation_of_5_3_and_13_should_be_equal_8() {
        let base = 5;
        let exponent = 3;
        let modulus = 13;

        let ans: i32 = ModExp::mod_exp(base,exponent, modulus);

        assert_eq!(ans, 8);
    }
}