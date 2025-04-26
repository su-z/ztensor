use std::ops::{Add, Div, Mul, Rem, Sub};

use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Num, One, Unsigned, Zero};

/// Unsigned integers which can be infinity (ω).
/// This module implements a representation of natural numbers extended with infinity.

/// Trait for types that can represent infinity (ω).
///
/// This trait defines methods for working with types that can have an
/// infinite value in addition to their regular numeric values.
pub trait Omega: Sized {
    /// Returns true if this value is infinity (ω).
    fn is_omega(&self) -> bool;
    
    /// Returns the infinity (ω) value for this type.
    fn omega() -> Self;
    
    /// Sets this value to infinity (ω).
    fn set_omega(&mut self){
        *self = Self::omega();
    }
}

/// An unsigned integer type extended with infinity (ω).
///
/// This enum represents either a natural number or infinity,
/// allowing arithmetic operations to work with potentially infinite values.
#[derive(PartialEq)]
pub enum OmegaUInt<N: Unsigned> {
    /// A regular natural number value
    Natural(N),
    /// The infinity value (ω)
    Omega
}
use OmegaUInt::*;

/// Implementation of the Omega trait for OmegaUInt.
impl<N: Unsigned> Omega for OmegaUInt<N> {
    fn is_omega(&self) -> bool{
        match self {
            Omega => true,
            _ => false
        }
    }
    fn omega() -> Self {
        return Omega;
    }
}

/// Implementation of Zero trait for OmegaUInt.
///
/// Defines what "zero" means in the context of omega extended integers.
/// Infinity is never zero.
impl<N: Unsigned + CheckedAdd> Zero for OmegaUInt<N> {
    fn is_zero(&self) -> bool {
        match self {
            Omega => false,
            Natural(x) => x.is_zero()
        }
    }
    fn zero() -> Self {
        return Natural(N::zero());
    }
}

/// Implementation of Num trait for OmegaUInt.
impl <N: Unsigned + CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + Copy> Num for OmegaUInt<N> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        panic!("Not supported!");
    }
}

/// Implementation of Unsigned trait for OmegaUInt.
impl <N: Unsigned + CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + Copy> Unsigned for OmegaUInt<N> {}

/// Implementation of One trait for OmegaUInt.
///
/// Defines what "one" means in the context of omega extended integers.
/// Infinity is never one.
impl<N: Unsigned + CheckedAdd + CheckedMul> One for OmegaUInt<N> {
    fn is_one(&self) -> bool {
        match self {
            Omega => false,
            Natural(x) => x.is_one()
        }
    }
    fn one() -> Self {
        return Natural(N::one());
    }
}

/// Helper function for checked operations on OmegaUInt values.
///
/// Handles common patterns in checked operations, including special cases for
/// infinity and zero values.
fn omega_uint_chkd_op<N: Unsigned, F: Fn(&N, &N) -> Option<N>>(
    lhs: &OmegaUInt<N>, 
    rhs: &OmegaUInt<N>, 
    op: F,
    value_if_only_lhs_omega: Option<OmegaUInt<N>>,
    value_if_only_rhs_omega: Option<OmegaUInt<N>>,
    value_if_lhs_rhs_omega: Option<OmegaUInt<N>>,
    chk_zero: bool,
    value_if_only_lhs_zero: Option<OmegaUInt<N>>,
    value_if_only_rhs_zero: Option<OmegaUInt<N>>,
    value_if_lhs_rhs_zero: Option<OmegaUInt<N>>,
    value_if_op_overflow: Option<OmegaUInt<N>>
) -> Option<OmegaUInt<N>>{
    let is_lhs_omega = match lhs {
        Omega => true, Natural(_) => false
    };
    let is_rhs_omega = match rhs {
        Omega => true, Natural(_) => false
    };
    if is_lhs_omega && !is_rhs_omega {
        return value_if_only_lhs_omega;
    }
    if !is_lhs_omega && is_rhs_omega {
        return value_if_only_rhs_omega;
    }
    if is_lhs_omega && is_rhs_omega {
        return value_if_lhs_rhs_omega;
    }
    // Proper operation    
    let lhs_value = match lhs {
        Natural(x) => x,
        Omega => panic!()
    };
    let rhs_value = match rhs {
        Natural(x) => x,
        Omega => panic!()
    };
    if chk_zero {
        if *lhs_value == N::zero() && *rhs_value != N::zero() {
            return value_if_only_lhs_zero;
        }
        if *lhs_value != N::zero() && *rhs_value == N::zero() {
            return value_if_only_rhs_zero;
        }
        if *lhs_value == N::zero() && *rhs_value == N::zero() {
            return value_if_lhs_rhs_zero;
        }
    }
    let res = match op(lhs_value, rhs_value){
        Some(x) => Natural(x),
        None => return value_if_op_overflow
    };
    return Some(res);
}

/// Implementation of division for OmegaUInt.
impl<N: Unsigned + CheckedDiv + Copy> Div for OmegaUInt<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self.checked_div(&rhs) {
            Some(y) => return y,
            None => panic!()
        }
    }
}

/// Implementation of checked division for OmegaUInt.
impl<N: Unsigned + CheckedDiv + Copy> CheckedDiv for OmegaUInt<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        return omega_uint_chkd_op(self, v, 
            N::checked_div, 
            Some(Omega), 
            Some(Natural(N::zero())),
            None,
            true, 
            Some(Natural(N::zero())), 
            None, 
            None, 
            None
        );
    }
}

/// Implementation of remainder operation for OmegaUInt.
impl<N: Unsigned> Rem for OmegaUInt<N> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let self_value: N;
        let rhs_value: N;
        match self {
            Self::Natural(x) => self_value = x,
            Self::Omega => panic!()
        };
        match rhs {
            Self::Natural(x) => rhs_value = x,
            Self::Omega => panic!()
        };
        return Self::Natural(self_value % rhs_value);
    }
}

/// Implementation of addition for OmegaUInt.
impl<N: Unsigned + CheckedAdd> Add for OmegaUInt<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self.checked_add(&rhs) {
            Some(y) => return y,
            None => panic!("Overflow!")
        }
    }
}

/// Implementation of checked addition for OmegaUInt.
impl<N: Unsigned + CheckedAdd> CheckedAdd for OmegaUInt<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        return omega_uint_chkd_op(self, v, 
            N::checked_add, 
            Some(Omega), 
            Some(Omega), 
            Some(Omega), 
            false,
            None,
            None, 
            None, 
            Some(Omega));
    }
}

/// Implementation of checked subtraction for OmegaUInt.
impl<N: Unsigned + CheckedSub> CheckedSub for OmegaUInt<N>{
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        return omega_uint_chkd_op(self, v, 
            N::checked_sub, 
            Some(Omega), 
            None, 
            None, 
            false, None, None, None, None);
    }
}

/// Implementation of subtraction for OmegaUInt.
impl<N: Unsigned + CheckedSub> Sub for OmegaUInt<N>{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self.checked_sub(&rhs) {
            Some(y) => return y,
            None => panic!()
        }
    }
}

/// Implementation of checked multiplication for OmegaUInt.
impl<N: Unsigned + CheckedAdd + CheckedMul> CheckedMul for OmegaUInt<N>{
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        if (self.is_zero()&&v.is_omega()) || (self.is_omega()&&v.is_zero()){
            return None;
        }
        return omega_uint_chkd_op(self, v, 
            N::checked_mul, 
            Some(Omega), 
            None, 
            None, 
            false, None, None, None, None);
    }
}

/// Implementation of multiplication for OmegaUInt.
impl<N: Unsigned + CheckedAdd + CheckedMul> Mul for OmegaUInt<N>{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self.checked_mul(&rhs) {
            Some(y) => return y,
            None => panic!()
        }
    }
}

/// Implementation of From trait to convert from unsigned integers to OmegaUInt.
impl<U: Unsigned> From<U> for OmegaUInt<U> {
    fn from(value: U) -> Self {
        OmegaUInt::Natural(value)
    }
}
