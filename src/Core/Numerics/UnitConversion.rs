use num_bigint::{BigInt, ToBigInt};
use std::str::FromStr;

pub struct UnitConversion;

impl UnitConversion {
    fn get_multiplier(units: i32) -> BigInt {
        let mut unit_multiplier = BigInt::one();
        for _ in 0..units {
            unit_multiplier *= 10;
        }
        unit_multiplier
    }

    pub fn to_decimal(value: &BigInt, units: i32) -> f64 {
        if *value == BigInt::zero() {
            return 0.0;
        }
        if units == 0 {
            return value.to_i64().unwrap() as f64;
        }
        let multiplier = Self::get_multiplier(units).to_f64().unwrap();
        let n = f64::from_str(&value.to_str_radix(10)).unwrap();
        n / multiplier
    }

    pub fn to_decimal_str(value: &str, units: i32) -> f64 {
        if value.is_empty() {
            return 0.0;
        }
        let big = BigInt::from_str(value).unwrap();
        Self::to_decimal(&big, units)
    }

    pub fn to_big_integer(n: f64, units: i32) -> BigInt {
        let multiplier = Self::get_multiplier(units);
        let a = BigInt::from(n as i64);
        let b = multiplier.clone();
        let frac_part = n - n.floor();
        let mut c = BigInt::zero();
        if frac_part > 0.0 {
            let l = frac_part * multiplier.to_f64().unwrap();
            c = BigInt::from(l as i64);
        }
        a * b + c
    }

    pub fn convert_decimals(value: &BigInt, decimal_from: i32, decimal_to: i32) -> BigInt {
        if decimal_from == decimal_to {
            return value.clone();
        }
        let from_factor = BigInt::from(10).pow(decimal_from as u32);
        let to_factor = BigInt::from(10).pow(decimal_to as u32);
        let output = value * &to_factor / from_factor;
        output
    }

    pub fn get_unit_value(decimals: i32) -> BigInt {
        Self::to_big_integer(1.0, decimals)
    }
}
