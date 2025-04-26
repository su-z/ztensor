/// This module defines the OmegaInt type, which represents integers with two special values: POmega and MOmega.
/// These special values are used to represent positive and negative infinity, respectively.
/// The module also implements various traits for OmegaInt, including arithmetic operations and comparisons.

use std::ops::{Add, Div, Mul, Neg, Sub, Rem};

use num_traits::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Signed, One, Zero, Num};

// Unsigned integers which can be infinity

/// This type represents the sign of a number.
/// It can be 1 for positive, -1 for negative, or 0 for zero.
pub type Sign = i8;

/// This trait defines the behavior of a type that can be represented as a positive or negative infinity.
/// It provides methods to check if the value is positive or negative infinity,
/// and to get the corresponding positive or negative infinity value.
/// The trait is implemented for the OmegaInt type, which represents integers with two special values: POmega and MOmega.
/// The POmega and MOmega values are used to represent positive and negative infinity, respectively.
pub trait PMOmega: Sized {
    fn is_pmomega(&self) -> Sign;
    fn pomega() -> Self;
    fn momega() -> Self;
}

/// This enum represents an integer value, positive infinity (POmega), or negative infinity (MOmega).
/// The enum is generic over a type N, which represents the integer type.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum OmegaInt<N> {
    Integer(N),
    /// Represents positive infinity.
    POmega,
    /// Represents negative infinity.
    MOmega
}
pub use OmegaInt::*;

impl<N> PMOmega for OmegaInt<N> {
    fn is_pmomega(&self) -> Sign{
        match self {
            POmega => 1,
            MOmega => -1,
            _ => 0
        }
    }
    fn pomega() -> Self {
        return POmega;
    }
    fn momega() -> Self {
        return MOmega;
    }
}

/// This trait defines the behavior of a type that can provide its sign.
pub trait GetSign {
    fn get_sign(&self) -> Sign;
} 

/// This trait defines the behavior of a type that can provide its sign.
pub trait PrimGetSign {
    fn get_sign(&self) -> Sign;
} 

impl<T: Signed> PrimGetSign for T {
    fn get_sign(&self) -> Sign {
        if self.is_positive() {
            return 1;
        }
        if self.is_negative() {
            return -1;
        }
        return 0;
    }
}

impl<N: PrimGetSign> GetSign for OmegaInt<N> {
    fn get_sign(&self) -> Sign {
        match self {
            POmega => 1,
            MOmega => -1,
            Integer(x) => x.get_sign()
        }
    }
}

impl<N: CheckedAdd + PrimGetSign + Zero> Zero for OmegaInt<N> {
    fn is_zero(&self) -> bool {
        match self {
            Integer(x) => x.is_zero(),
            _ => false
        }
    }
    fn zero() -> Self {
        return Integer(N::zero());
    }
}

impl<N: CheckedAdd + CheckedMul + PrimGetSign + One + PartialEq> One for OmegaInt<N> {
    fn is_one(&self) -> bool {
        match self {
            Integer(x) => x.is_one(),
            _ => false
        }
    }
    fn one() -> Self {
        return Integer(N::one());
    }
}

fn omega_int_chkd_op<N: PrimGetSign, F: Fn(&N, &N) -> Option<N>, OC: Fn(Sign, Sign) -> Option<OmegaInt<N>>, SC: Fn(Sign, Sign) -> Option<OmegaInt<N>>>(
    lhs: &OmegaInt<N>, 
    rhs: &OmegaInt<N>, 
    op: F,
    omega_checker: OC,
    chk_zero: bool,
    sign_checker: SC,
    value_if_op_overflow: Option<OmegaInt<N>>
) -> Option<OmegaInt<N>>{
    let (lhs_osp, rhs_osp) = (lhs.is_pmomega(), rhs.is_pmomega());
    if (lhs_osp, rhs_osp) != (0,0) {
        return omega_checker(lhs_osp, rhs_osp);
    }
    let lhs_value = match lhs {
        Integer(x) => x,
        _ => panic!()
    };
    let rhs_value = match rhs {
        Integer(x) => x,
        _ => panic!()
    };
    if chk_zero {
        let (lhs_zero, rhs_zero) = (lhs_value.get_sign(), rhs.get_sign());
        if !((lhs_zero!=0)&&(rhs_zero!=0)) {
            return sign_checker(lhs_zero, rhs_zero);
        }
    }
    match op(lhs_value, rhs_value) {
        Some(x) => return Some(Integer(x)),
        None => return value_if_op_overflow
    }
}

fn empty_sign_checker<N>(_:Sign, _:Sign) -> Option<OmegaInt<N>>{None}

impl<N: CheckedAdd + PrimGetSign> Add for OmegaInt<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self.checked_add(&rhs) {
            Some(y) => return y,
            None => panic!("Overflow!")
        }
    }
}

impl<N: CheckedAdd + PrimGetSign> CheckedAdd for OmegaInt<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        fn omega_checker<N: CheckedAdd + PrimGetSign>(s0: Sign, s1: Sign) -> Option<OmegaInt<N>>{
            if s0 == 0 {
                if s1 == 1 {
                    return Some(POmega);
                }
                if s1 == -1 {
                    return Some(MOmega);
                }
                panic!()
            }
            if s1 == 0 {
                if s0 == 1 {
                    return Some(POmega);
                }
                if s0 == -1 {
                    return Some(MOmega);
                }
                panic!()
            }
            if (s0 == 1) & (s1 == 1) {
                return Some(POmega);
            }
            if (s0 == -1) & (s1 == -1) {
                return Some(POmega);
            }
            return None;
        }
        return omega_int_chkd_op(self, v, 
            N::checked_add, 
            omega_checker, 
            false, empty_sign_checker::<N>, 
            None
        );
    }
}

impl<N: CheckedSub + PrimGetSign> Sub for OmegaInt<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self.checked_sub(&rhs) {
            Some(y) => return y,
            None => panic!("Overflow!")
        }
    }
}

impl<N: CheckedSub + PrimGetSign> CheckedSub for OmegaInt<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        fn omega_checker<N: CheckedSub>(s0: Sign, s1: Sign) -> Option<OmegaInt<N>>{
            if s0 == 0 {
                if s1 == 1 {
                    return Some(MOmega);
                }
                if s1 == -1 {
                    return Some(POmega);
                }
                panic!()
            }
            if s1 == 0 {
                if s0 == 1 {
                    return Some(POmega);
                }
                if s0 == -1 {
                    return Some(MOmega);
                }
                panic!()
            }
            if (s0 == 1) & (s1 == -1) {
                return Some(POmega);
            }
            if (s0 == -1) & (s1 == 1) {
                return Some(POmega);
            }
            return None;
        }
        return omega_int_chkd_op(self, v, 
            N::checked_sub, 
            omega_checker, 
            false, empty_sign_checker::<N>, 
            None
        );
    }
}

impl<N: CheckedMul + PrimGetSign> Mul for OmegaInt<N> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match self.checked_mul(&rhs) {
            Some(y) => return y,
            None => panic!("Overflow!")
        }
    }
}

impl<N: CheckedMul + PrimGetSign> CheckedMul for OmegaInt<N> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        match self {
            POmega => match v {
                POmega => Some(POmega),
                MOmega => Some(MOmega),
                Integer(x) => {
                    match x.get_sign() {
                        1 => Some(POmega),
                        -1 => Some(MOmega),
                        0 => None,
                        _ => panic!()
                    }
                }
            },
            MOmega => match v {
                POmega => Some(MOmega),
                MOmega => Some(POmega),
                Integer(x) => {
                    match x.get_sign() {
                        1 => Some(MOmega),
                        -1 => Some(POmega),
                        0 => None,
                        _ => panic!()
                    }
                }
            },
            Integer(x) => match v {
                POmega => {
                    match x.get_sign() {
                        1 => Some(POmega),
                        -1 => Some(MOmega),
                        0 => None,
                        _ => panic!()
                    }
                },
                MOmega => {
                    match x.get_sign() {
                        1 => Some(MOmega),
                        -1 => Some(POmega),
                        0 => None,
                        _ => panic!()
                    }
                },
                Integer(y) => {
                    match x.checked_mul(y){
                        Some(s) => Some(Integer(s)),
                        None => None
                    }
                }
            }
        }
    }
}


impl<N: CheckedDiv + CheckedAdd + PrimGetSign + Zero> Div for OmegaInt<N> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match self.checked_div(&rhs) {
            Some(y) => return y,
            None => panic!("Overflow!")
        }
    }
}

impl<N: CheckedDiv + CheckedAdd + PrimGetSign + Zero> CheckedDiv for OmegaInt<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        match self {
            POmega => match v {
                POmega => None,
                MOmega => None,
                Integer(x) => {
                    match x.get_sign() {
                        1 => Some(POmega),
                        -1 => Some(MOmega),
                        0 => None,
                        _ => panic!()
                    }
                }
            },
            MOmega => match v {
                POmega => None,
                MOmega => None,
                Integer(x) => {
                    match x.get_sign() {
                        1 => Some(MOmega),
                        -1 => Some(POmega),
                        0 => None,
                        _ => panic!()
                    }
                }
            },
            Integer(x) => match v {
                POmega => {
                    Some(Self::zero())
                },
                MOmega => {
                    Some(Self::zero())
                },
                Integer(y) => {
                    match x.checked_div(y){
                        Some(s) => Some(Integer(s)),
                        None => None
                    }
                }
            }
        }
    }
}

impl <N: Neg<Output = N>> Neg for OmegaInt<N> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            POmega => MOmega,
            MOmega => POmega,
            Integer(x) => Integer(-x)
        }
    }
}

impl <N: CheckedAdd + CheckedSub + CheckedMul + CheckedDiv + PrimGetSign + Copy + PartialEq + Zero + One + Neg<Output = N> + Rem<Output = N>> Signed for OmegaInt<N> {
    fn abs(&self) -> Self {
        match GetSign::get_sign(self) {
            1 => self.clone(),
            _ => -*self
        }
    }
    fn abs_sub(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
    fn signum(&self) -> Self {
        match GetSign::get_sign(self) {
            1 => Integer(N::one()),
            -1 => Integer(-N::one()),
            0 => Integer(N::zero()),
            _ => panic!()
        }
    }
    fn is_negative(&self) -> bool {
        GetSign::get_sign(self) == -1
    }
    fn is_positive(&self) -> bool {
        GetSign::get_sign(self) == 1
    }
}

impl<N: PrimGetSign + Rem<Output = N>> Rem for OmegaInt<N> {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let self_value: N;
        let rhs_value: N;
        match self {
            Integer(x) => self_value = x,
            _ => panic!()
        };
        match rhs {
            Integer(x) => rhs_value = x,
            _ => panic!()
        };
        return Integer(self_value % rhs_value);
    }
}

impl <N: CheckedAdd + CheckedMul + CheckedSub + CheckedDiv + Copy + PrimGetSign + PartialEq + Zero + One + Rem<Output = N>> Num for OmegaInt<N> {
    type FromStrRadixErr = ();
    fn from_str_radix(_str: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        panic!()
    }
}

#[test]
fn test_omega_int(){
    let x: OmegaInt<i32> = POmega;
    assert_eq!(x + x, x);
    let y: OmegaInt<i32> = Integer(89);
    assert_eq!(y + y, Integer(89*2));
    assert_eq!(x + y, POmega);
    assert_eq!(y - x, MOmega);
    assert_eq!(-x, MOmega);
    assert_eq!(x.checked_sub(&x), None);
    assert_eq!(x*Integer(-92), MOmega);
}


impl<I> From<I> for OmegaInt<I> {
    fn from(value: I) -> Self {
        Self::Integer(value.into())
    }
}
