//TODO test

use malachite_base::misc::{
    CheckedFrom, CheckedInto, Max, Min, Named, Walkable, WrappingFrom, WrappingInto,
};
use malachite_base::num::{
    BitAccess, BitScan, CeilingDivAssignNegMod, CeilingDivNegMod, CheckedAdd, CheckedDiv,
    CheckedMul, CheckedNeg, CheckedRem, CheckedShl, CheckedShr, CheckedSub, CountOnes, CountZeros,
    DivAssignMod, DivAssignRem, DivExact, DivExactAssign, DivMod, DivRem, DivRound, DivRoundAssign,
    DivisibleBy, DivisibleByPowerOfTwo, Endian, EqMod, EqModPowerOfTwo, HammingDistance,
    LeadingZeros, Mod, ModAssign, ModPowerOfTwo, NegMod, NegModAssign, NotAssign, One, OrdAbs,
    OverflowingAdd, OverflowingAddAssign, OverflowingDiv, OverflowingDivAssign, OverflowingMul,
    OverflowingMulAssign, OverflowingNeg, OverflowingNegAssign, OverflowingRem,
    OverflowingRemAssign, OverflowingShl, OverflowingShr, OverflowingSub, OverflowingSubAssign,
    Parity, PartialOrdAbs, Pow, PrimitiveInteger, RotateLeft, RotateRight, SaturatingAdd,
    SaturatingAddAssign, SaturatingMul, SaturatingMulAssign, SaturatingSub, SaturatingSubAssign,
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, SignificantBits, TrailingZeros,
    WrappingAdd, WrappingAddAssign, WrappingDiv, WrappingDivAssign, WrappingMul, WrappingMulAssign,
    WrappingNeg, WrappingNegAssign, WrappingRem, WrappingRemAssign, WrappingShl, WrappingShr,
    WrappingSub, WrappingSubAssign, Zero,
};
use malachite_base::round::RoundingMode;
use rand::distributions::range::SampleRange;
use rand::distributions::Range;
use rand::{Rand, Rng};
use std::cmp::{min, Ordering};
use std::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};
use std::iter::{Product, Sum};
use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};
use std::str::FromStr;

macro_rules! shift_impl_integer {
    ($u:ident, $t:ident) => {
        impl Shl<$t> for $u {
            type Output = $u;

            #[inline]
            fn shl(self, pow: $t) -> $u {
                $u((self.0 << pow) & $u::MASK)
            }
        }

        impl Shr<$t> for $u {
            type Output = $u;

            #[inline]
            fn shr(self, pow: $t) -> $u {
                $u(self.0 >> pow)
            }
        }

        impl ShlAssign<$t> for $u {
            #[inline]
            fn shl_assign(&mut self, pow: $t) {
                self.0 <<= pow;
                self.0 &= $u::MASK;
            }
        }

        impl ShrAssign<$t> for $u {
            #[inline]
            fn shr_assign(&mut self, pow: $t) {
                self.0 >>= pow;
            }
        }

        impl ShrRound<$t> for $u {
            type Output = $u;

            #[inline]
            fn shr_round(self, pow: $t, rm: RoundingMode) -> $u {
                $u(self.0.shr_round(pow, rm))
            }
        }

        impl ShrRoundAssign<$t> for $u {
            #[inline]
            fn shr_round_assign(&mut self, pow: $t, rm: RoundingMode) {
                self.0.shr_round_assign(pow, rm);
            }
        }
    };
}

macro_rules! shift_impl_signed {
    ($u:ident, $t:ident) => {
        shift_impl_integer!($u, $t);

        impl ShlRound<$t> for $u {
            type Output = $u;

            #[inline]
            fn shl_round(self, pow: $t, rm: RoundingMode) -> $u {
                $u(self.0.shl_round(pow, rm) & $u::MASK)
            }
        }

        impl ShlRoundAssign<$t> for $u {
            #[inline]
            fn shl_round_assign(&mut self, pow: $t, rm: RoundingMode) {
                self.0.shl_round_assign(pow, rm);
                self.0 &= $u::MASK;
            }
        }
    };
}

macro_rules! conversions_impl {
    ($u:ident, $base:ident, $t:ident) => {
        impl WrappingFrom<$t> for $u {
            #[inline]
            fn wrapping_from(value: $t) -> $u {
                $u($base::wrapping_from(value) & $u::MASK)
            }
        }

        impl WrappingInto<$t> for $u {
            #[inline]
            fn wrapping_into(self) -> $t {
                $t::wrapping_from(self.0)
            }
        }

        impl CheckedFrom<$t> for $u {
            #[inline]
            fn checked_from(value: $t) -> Option<$u> {
                $base::checked_from(value)
                    .and_then(|x| if x > $u::MASK { None } else { Some($u(x)) })
            }
        }

        impl CheckedInto<$t> for $u {
            #[inline]
            fn checked_into(self) -> Option<$t> {
                self.0.checked_into()
            }
        }
    };
}

macro_rules! uint {
    ($u:ident, $base:ident, $bits:expr) => {
        #[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $u(pub $base);

        impl $u {
            const MASK: $base = (1 << $bits) - 1;
        }

        impl From<$u> for $base {
            #[inline]
            fn from(value: $u) -> $base {
                value.0
            }
        }

        impl Rand for $u {
            #[inline]
            fn rand<R: Rng>(rng: &mut R) -> $u {
                $u($base::rand(rng) & $u::MASK)
            }
        }

        impl SampleRange for $u {
            #[inline]
            fn construct_range(_low: Self, _high: Self) -> Range<Self> {
                unimplemented!();
            }

            #[inline]
            fn sample_range<R: Rng>(_r: &Range<Self>, _rng: &mut R) -> Self {
                unimplemented!();
            }
        }

        impl UpperHex for $u {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                UpperHex::fmt(&self.0, f)
            }
        }

        impl LowerHex for $u {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                LowerHex::fmt(&self.0, f)
            }
        }

        impl Binary for $u {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Binary::fmt(&self.0, f)
            }
        }

        impl Display for $u {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl Octal for $u {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
                Octal::fmt(&self.0, f)
            }
        }

        impl FromStr for $u {
            type Err = ();

            #[inline]
            fn from_str(src: &str) -> Result<Self, ()> {
                $base::from_str(src).map_err(|_| ()).and_then(|x| {
                    if x <= $u::MASK {
                        Ok($u(x))
                    } else {
                        Err(())
                    }
                })
            }
        }

        impl Product for $u {
            #[inline]
            fn product<I: Iterator>(_iter: I) -> Self {
                unimplemented!();
            }
        }

        impl Sum for $u {
            #[inline]
            fn sum<I: Iterator>(_iter: I) -> Self {
                unimplemented!();
            }
        }

        shift_impl_integer!($u, u8);
        shift_impl_integer!($u, u16);
        shift_impl_integer!($u, u32);
        shift_impl_integer!($u, u64);
        shift_impl_signed!($u, i8);
        shift_impl_signed!($u, i16);
        shift_impl_signed!($u, i32);
        shift_impl_signed!($u, i64);

        impl BitAndAssign for $u {
            #[inline]
            fn bitand_assign(&mut self, rhs: $u) {
                self.0 &= rhs.0;
            }
        }

        impl BitOrAssign for $u {
            #[inline]
            fn bitor_assign(&mut self, rhs: $u) {
                self.0 |= rhs.0;
                self.0 &= $u::MASK;
            }
        }

        impl BitXorAssign for $u {
            #[inline]
            fn bitxor_assign(&mut self, rhs: $u) {
                self.0 ^= rhs.0;
                self.0 &= $u::MASK;
            }
        }

        impl BitAnd for $u {
            type Output = $u;

            #[inline]
            fn bitand(mut self, rhs: $u) -> $u {
                self &= rhs;
                self
            }
        }

        impl BitOr for $u {
            type Output = $u;

            #[inline]
            fn bitor(mut self, rhs: $u) -> $u {
                self |= rhs;
                self
            }
        }

        impl BitXor for $u {
            type Output = $u;

            #[inline]
            fn bitxor(mut self, rhs: $u) -> $u {
                self ^= rhs;
                self
            }
        }

        impl Not for $u {
            type Output = $u;

            #[inline]
            fn not(self) -> $u {
                $u(!self.0 & $u::MASK)
            }
        }

        impl Add for $u {
            type Output = $u;

            #[inline]
            fn add(self, rhs: $u) -> $u {
                self.checked_add(rhs).expect("Addition overflow")
            }
        }

        impl Sub for $u {
            type Output = $u;

            #[inline]
            fn sub(self, rhs: $u) -> $u {
                self.checked_sub(rhs).expect("Subtraction underflow")
            }
        }

        impl Mul for $u {
            type Output = $u;

            #[inline]
            fn mul(self, rhs: $u) -> $u {
                self.checked_mul(rhs).expect("Multiplication overflow")
            }
        }

        impl Div for $u {
            type Output = $u;

            #[inline]
            fn div(self, rhs: $u) -> $u {
                $u(self.0 / rhs.0)
            }
        }

        impl Rem for $u {
            type Output = $u;

            #[inline]
            fn rem(self, rhs: $u) -> $u {
                $u(self.0 % rhs.0)
            }
        }

        impl AddAssign for $u {
            #[inline]
            fn add_assign(&mut self, rhs: $u) {
                *self = *self + rhs;
            }
        }

        impl SubAssign for $u {
            #[inline]
            fn sub_assign(&mut self, rhs: $u) {
                *self = *self - rhs;
            }
        }

        impl MulAssign for $u {
            #[inline]
            fn mul_assign(&mut self, rhs: $u) {
                *self = *self * rhs;
            }
        }

        impl DivAssign for $u {
            #[inline]
            fn div_assign(&mut self, rhs: $u) {
                self.0 /= rhs.0;
            }
        }

        impl RemAssign for $u {
            #[inline]
            fn rem_assign(&mut self, rhs: $u) {
                self.0 %= rhs.0;
            }
        }

        impl HammingDistance<$u> for $u {
            #[inline]
            fn hamming_distance(self, rhs: $u) -> u64 {
                self.0.hamming_distance(rhs.0)
            }
        }

        impl SignificantBits for $u {
            #[inline]
            fn significant_bits(self) -> u64 {
                self.0.significant_bits()
            }
        }

        impl Zero for $u {
            const ZERO: $u = $u(0);
        }

        impl One for $u {
            const ONE: $u = $u(1);
        }

        impl PartialOrdAbs<$u> for $u {
            #[inline]
            fn partial_cmp_abs(&self, other: &$u) -> Option<Ordering> {
                self.0.partial_cmp_abs(&other.0)
            }
        }

        impl OrdAbs for $u {
            #[inline]
            fn cmp_abs(&self, other: &$u) -> Ordering {
                self.0.cmp_abs(&other.0)
            }
        }

        impl NotAssign for $u {
            #[inline]
            fn not_assign(&mut self) {
                self.0.not_assign();
                self.0 &= $u::MASK;
            }
        }

        impl BitAccess for $u {
            #[inline]
            fn get_bit(&self, index: u64) -> bool {
                self.0.get_bit(index)
            }

            #[inline]
            fn set_bit(&mut self, index: u64) {
                if index < $bits {
                    self.0 |= 1 << index;
                } else {
                    panic!(
                        "Cannot set bit {} in non-negative value of width {}",
                        index, $bits
                    );
                }
            }

            #[inline]
            fn clear_bit(&mut self, index: u64) {
                self.0.clear_bit(index)
            }
        }

        impl DivisibleBy for $u {
            #[inline]
            fn divisible_by(self, rhs: $u) -> bool {
                self.0.divisible_by(rhs.0)
            }
        }

        impl DivExact for $u {
            type Output = $u;

            #[inline]
            fn div_exact(self, rhs: $u) -> $u {
                self / rhs
            }
        }

        impl DivExactAssign for $u {
            #[inline]
            fn div_exact_assign(&mut self, rhs: $u) {
                *self /= rhs;
            }
        }

        impl Mod for $u {
            type Output = $u;

            #[inline]
            fn mod_op(self, rhs: $u) -> $u {
                self % rhs
            }
        }

        impl NegMod for $u {
            type Output = $u;

            #[inline]
            fn neg_mod(self, rhs: $u) -> $u {
                $u(self.0.neg_mod(rhs.0))
            }
        }

        impl CeilingDivNegMod for $u {
            type DivOutput = $u;
            type ModOutput = $u;

            #[inline]
            fn ceiling_div_neg_mod(self, rhs: $u) -> ($u, $u) {
                let (quotient, remainder) = self.0.ceiling_div_neg_mod(rhs.0);
                ($u(quotient), $u(remainder))
            }
        }

        impl ModAssign for $u {
            #[inline]
            fn mod_assign(&mut self, rhs: $u) {
                *self %= rhs;
            }
        }

        impl NegModAssign for $u {
            #[inline]
            fn neg_mod_assign(&mut self, rhs: $u) {
                self.0.neg_mod_assign(rhs.0);
            }
        }

        impl CeilingDivAssignNegMod for $u {
            type ModOutput = $u;

            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, rhs: $u) -> $u {
                $u(self.0.ceiling_div_assign_neg_mod(rhs.0))
            }
        }

        impl DivRem for $u {
            type DivOutput = $u;
            type RemOutput = $u;

            #[inline]
            fn div_rem(self, rhs: $u) -> ($u, $u) {
                let (quotient, remainder) = self.0.div_rem(rhs.0);
                ($u(quotient), $u(remainder))
            }
        }

        impl DivAssignRem for $u {
            type RemOutput = $u;

            #[inline]
            fn div_assign_rem(&mut self, rhs: $u) -> $u {
                $u(self.0.div_assign_rem(rhs.0))
            }
        }

        impl DivMod for $u {
            type DivOutput = $u;
            type ModOutput = $u;

            #[inline]
            fn div_mod(self, rhs: $u) -> ($u, $u) {
                let (quotient, remainder) = self.0.div_mod(rhs.0);
                ($u(quotient), $u(remainder))
            }
        }

        impl DivAssignMod for $u {
            type ModOutput = $u;

            #[inline]
            fn div_assign_mod(&mut self, rhs: $u) -> $u {
                $u(self.0.div_assign_mod(rhs.0))
            }
        }

        impl DivisibleByPowerOfTwo for $u {
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.0.divisible_by_power_of_two(pow)
            }
        }

        impl Parity for $u {
            #[inline]
            fn even(self) -> bool {
                self.0.even()
            }

            #[inline]
            fn odd(self) -> bool {
                self.0.even()
            }
        }

        impl BitScan for $u {
            #[inline]
            fn index_of_next_false_bit(self, index: u64) -> Option<u64> {
                self.0.index_of_next_false_bit(index)
            }

            #[inline]
            fn index_of_next_true_bit(self, index: u64) -> Option<u64> {
                self.0.index_of_next_true_bit(index)
            }
        }

        impl WrappingNegAssign for $u {
            #[inline]
            fn wrapping_neg_assign(&mut self) {
                self.0.wrapping_neg_assign();
                self.0 &= $u::MASK;
            }
        }

        impl EqModPowerOfTwo for $u {
            #[inline]
            fn eq_mod_power_of_two(self, other: $u, pow: u64) -> bool {
                self.0.eq_mod_power_of_two(other.0, pow)
            }
        }

        impl Pow<u32> for $u {
            type Output = $u;

            #[inline]
            fn pow(self, _pow: u32) -> $u {
                unimplemented!();
            }
        }

        impl OverflowingShl for $u {
            type Output = $u;

            #[inline]
            fn overflowing_shl(self, _pow: u32) -> ($u, bool) {
                unimplemented!();
            }
        }

        impl OverflowingShr for $u {
            type Output = $u;

            #[inline]
            fn overflowing_shr(self, _pow: u32) -> ($u, bool) {
                unimplemented!();
            }
        }

        impl OverflowingNeg for $u {
            type Output = $u;

            #[inline]
            fn overflowing_neg(self) -> ($u, bool) {
                unimplemented!();
            }
        }

        impl OverflowingRem for $u {
            type Output = $u;

            #[inline]
            fn overflowing_rem(self, _other: $u) -> ($u, bool) {
                unimplemented!();
            }
        }

        impl OverflowingDiv for $u {
            type Output = $u;

            #[inline]
            fn overflowing_div(self, _other: $u) -> ($u, bool) {
                unimplemented!();
            }
        }

        impl OverflowingMul for $u {
            type Output = $u;

            #[inline]
            fn overflowing_mul(self, other: $u) -> ($u, bool) {
                let (product, mut overflow) = self.0.overflowing_mul(other.0);
                if !overflow && product > $u::MASK {
                    overflow = true;
                }
                ($u(product & $u::MASK), overflow)
            }
        }

        impl OverflowingSub for $u {
            type Output = $u;

            #[inline]
            fn overflowing_sub(self, other: $u) -> ($u, bool) {
                let (difference, mut overflow) = self.0.overflowing_sub(other.0);
                if !overflow && difference > $u::MASK {
                    overflow = true;
                }
                ($u(difference & $u::MASK), overflow)
            }
        }

        impl OverflowingAdd for $u {
            type Output = $u;

            #[inline]
            fn overflowing_add(self, other: $u) -> ($u, bool) {
                let (sum, mut overflow) = self.0.overflowing_add(other.0);
                if !overflow && sum > $u::MASK {
                    overflow = true;
                }
                ($u(sum & $u::MASK), overflow)
            }
        }

        impl WrappingShl for $u {
            type Output = $u;

            #[inline]
            fn wrapping_shl(self, _other: u32) -> $u {
                unimplemented!();
            }
        }

        impl WrappingShr for $u {
            type Output = $u;

            #[inline]
            fn wrapping_shr(self, _other: u32) -> $u {
                unimplemented!();
            }
        }

        impl WrappingSub for $u {
            type Output = $u;

            #[inline]
            fn wrapping_sub(self, other: $u) -> $u {
                $u(self.0.wrapping_sub(other.0) & $u::MASK)
            }
        }

        impl WrappingNeg for $u {
            type Output = $u;

            #[inline]
            fn wrapping_neg(self) -> $u {
                $u(self.0.wrapping_neg() & $u::MASK)
            }
        }

        impl WrappingRem for $u {
            type Output = $u;

            #[inline]
            fn wrapping_rem(self, other: $u) -> $u {
                self % other
            }
        }

        impl WrappingDiv for $u {
            type Output = $u;

            #[inline]
            fn wrapping_div(self, other: $u) -> $u {
                self / other
            }
        }

        impl WrappingAdd for $u {
            type Output = $u;

            #[inline]
            fn wrapping_add(self, other: $u) -> $u {
                $u(self.0.wrapping_add(other.0) & $u::MASK)
            }
        }

        impl WrappingMul for $u {
            type Output = $u;

            #[inline]
            fn wrapping_mul(self, other: $u) -> $u {
                $u(self.0.wrapping_mul(other.0) & $u::MASK)
            }
        }

        impl SaturatingAdd for $u {
            type Output = $u;

            #[inline]
            fn saturating_add(self, other: $u) -> $u {
                let sum = self.0.saturating_add(other.0);
                $u(if sum > $u::MASK { $u::MASK } else { sum })
            }
        }

        impl SaturatingMul for $u {
            type Output = $u;

            #[inline]
            fn saturating_mul(self, other: $u) -> $u {
                let product = self.0.saturating_mul(other.0);
                $u(if product > $u::MASK {
                    $u::MASK
                } else {
                    product
                })
            }
        }

        impl SaturatingSub for $u {
            type Output = $u;

            #[inline]
            fn saturating_sub(self, other: $u) -> $u {
                $u(self.0.saturating_sub(other.0))
            }
        }

        impl CheckedShr for $u {
            type Output = $u;

            #[inline]
            fn checked_shr(self, _other: u32) -> Option<$u> {
                unimplemented!();
            }
        }

        impl CheckedShl for $u {
            type Output = $u;

            #[inline]
            fn checked_shl(self, _other: u32) -> Option<$u> {
                unimplemented!();
            }
        }

        impl CheckedNeg for $u {
            type Output = $u;

            #[inline]
            fn checked_neg(self) -> Option<$u> {
                unimplemented!();
            }
        }

        impl CheckedRem for $u {
            type Output = $u;

            #[inline]
            fn checked_rem(self, other: $u) -> Option<$u> {
                Some(self % other)
            }
        }

        impl CheckedDiv for $u {
            type Output = $u;

            #[inline]
            fn checked_div(self, other: $u) -> Option<$u> {
                Some(self / other)
            }
        }

        impl CheckedMul for $u {
            type Output = $u;

            #[inline]
            fn checked_mul(self, other: $u) -> Option<$u> {
                self.0
                    .checked_mul(other.0)
                    .and_then(|x| if x > $u::MASK { None } else { Some($u(x)) })
            }
        }

        impl CheckedSub for $u {
            type Output = $u;

            #[inline]
            fn checked_sub(self, other: $u) -> Option<$u> {
                self.0.checked_sub(other.0).map(|x| $u(x))
            }
        }

        impl CheckedAdd for $u {
            type Output = $u;

            #[inline]
            fn checked_add(self, other: $u) -> Option<$u> {
                self.0
                    .checked_add(other.0)
                    .and_then(|x| if x > $u::MASK { None } else { Some($u(x)) })
            }
        }

        impl OverflowingNegAssign for $u {
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                unimplemented!();
            }
        }

        impl OverflowingRemAssign for $u {
            #[inline]
            fn overflowing_rem_assign(&mut self, _other: $u) -> bool {
                unimplemented!();
            }
        }

        impl OverflowingDivAssign for $u {
            #[inline]
            fn overflowing_div_assign(&mut self, _other: $u) -> bool {
                unimplemented!();
            }
        }

        impl OverflowingMulAssign for $u {
            #[inline]
            fn overflowing_mul_assign(&mut self, other: $u) -> bool {
                let mut overflow = self.0.overflowing_mul_assign(other.0);
                if !overflow && self.0 > $u::MASK {
                    overflow = true;
                }
                self.0 &= $u::MASK;
                overflow
            }
        }

        impl OverflowingSubAssign for $u {
            #[inline]
            fn overflowing_sub_assign(&mut self, other: $u) -> bool {
                let mut overflow = self.0.overflowing_sub_assign(other.0);
                if !overflow && self.0 > $u::MASK {
                    overflow = true;
                }
                self.0 &= $u::MASK;
                overflow
            }
        }

        impl OverflowingAddAssign for $u {
            #[inline]
            fn overflowing_add_assign(&mut self, other: $u) -> bool {
                let mut overflow = self.0.overflowing_add_assign(other.0);
                if !overflow && self.0 > $u::MASK {
                    overflow = true;
                }
                self.0 &= $u::MASK;
                overflow
            }
        }

        impl WrappingSubAssign for $u {
            #[inline]
            fn wrapping_sub_assign(&mut self, other: $u) {
                self.0.wrapping_sub_assign(other.0);
                self.0 &= $u::MASK;
            }
        }

        impl WrappingRemAssign for $u {
            #[inline]
            fn wrapping_rem_assign(&mut self, other: $u) {
                *self %= other;
            }
        }

        impl WrappingDivAssign for $u {
            #[inline]
            fn wrapping_div_assign(&mut self, other: $u) {
                *self /= other;
            }
        }

        impl WrappingAddAssign for $u {
            #[inline]
            fn wrapping_add_assign(&mut self, other: $u) {
                self.0.wrapping_add_assign(other.0);
                self.0 &= $u::MASK;
            }
        }

        impl WrappingMulAssign for $u {
            #[inline]
            fn wrapping_mul_assign(&mut self, other: $u) {
                self.0.wrapping_mul_assign(other.0);
                self.0 &= $u::MASK;
            }
        }

        impl SaturatingAddAssign for $u {
            #[inline]
            fn saturating_add_assign(&mut self, other: $u) {
                self.0.saturating_add_assign(other.0);
                if self.0 > $u::MASK {
                    self.0 = $u::MASK;
                }
            }
        }

        impl SaturatingMulAssign for $u {
            #[inline]
            fn saturating_mul_assign(&mut self, other: $u) {
                self.0.saturating_mul_assign(other.0);
                if self.0 > $u::MASK {
                    self.0 = $u::MASK;
                }
            }
        }

        impl SaturatingSubAssign for $u {
            #[inline]
            fn saturating_sub_assign(&mut self, other: $u) {
                self.0.saturating_sub_assign(other.0);
            }
        }

        impl Endian for $u {
            #[inline]
            fn swap_bytes(self) -> $u {
                unimplemented!();
            }

            #[inline]
            fn from_be(_x: $u) -> $u {
                unimplemented!();
            }

            #[inline]
            fn from_le(_x: $u) -> $u {
                unimplemented!();
            }

            #[inline]
            fn to_be(self) -> $u {
                unimplemented!();
            }

            #[inline]
            fn to_le(self) -> $u {
                unimplemented!();
            }
        }

        impl RotateLeft for $u {
            #[inline]
            fn rotate_left(self, _n: u32) -> $u {
                unimplemented!();
            }
        }

        impl RotateRight for $u {
            #[inline]
            fn rotate_right(self, _n: u32) -> $u {
                unimplemented!();
            }
        }

        impl TrailingZeros for $u {
            #[inline]
            fn trailing_zeros(self) -> u32 {
                min(self.0.trailing_zeros(), $bits)
            }
        }

        impl LeadingZeros for $u {
            #[inline]
            fn leading_zeros(self) -> u32 {
                if self.0 == 0 {
                    $bits
                } else {
                    self.0.trailing_zeros()
                }
            }
        }

        impl CountZeros for $u {
            #[inline]
            fn count_zeros(self) -> u32 {
                (self.0 | !$u::MASK).count_zeros()
            }
        }

        impl CountOnes for $u {
            #[inline]
            fn count_ones(self) -> u32 {
                self.0.count_ones()
            }
        }

        impl Walkable for $u {
            #[inline]
            fn increment(&mut self) {
                *self = self
                    .checked_add($u(1))
                    .expect("Cannot increment past the maximum value.");
            }

            #[inline]
            fn decrement(&mut self) {
                *self = self
                    .checked_sub($u(1))
                    .expect("Cannot decrement past the minimum value.");
            }
        }

        impl_named!($u);

        impl Max for $u {
            const MAX: $u = $u($u::MASK);
        }

        impl Min for $u {
            const MIN: $u = $u(0);
        }

        impl Bits for $u {
            #[inline]
            fn get_bits(self, high: u32, low: u32) -> $u {
                $u(self.0.get_bits(high, low))
            }

            #[inline]
            fn set_bits(&mut self, src: $u, high: u32, low: u32) {
                self.0.set_bits(src.0, high, low);
                self.0 &= $u::MASK;
            }
        }

        impl DivRound for $u {
            type Output = $u;

            #[inline]
            fn div_round(self, _other: $u, _rm: RoundingMode) -> $u {
                unimplemented!();
            }
        }

        impl DivRoundAssign for $u {
            #[inline]
            fn div_round_assign(&mut self, _other: $u, _rm: RoundingMode) {
                unimplemented!();
            }
        }

        impl EqMod<$u, $u> for $u {
            #[inline]
            fn eq_mod(self, _other: $u, _modulus: $u) -> bool {
                unimplemented!();
            }
        }

        conversions_impl!($u, $base, u8);
        conversions_impl!($u, $base, u16);
        conversions_impl!($u, $base, u32);
        conversions_impl!($u, $base, u64);
        conversions_impl!($u, $base, i8);
        conversions_impl!($u, $base, i16);
        conversions_impl!($u, $base, i32);
        conversions_impl!($u, $base, i64);

        impl PrimitiveInteger for $u {
            const LOG_WIDTH: u32 = 0; // not really
            const WIDTH: u32 = $bits;
        }
    };
}

uint!(U2, u8, 2);
uint!(U3, u8, 3);
uint!(U4, u8, 4);
uint!(U5, u8, 5);
uint!(U11, u16, 11);
uint!(U12, u16, 12);
uint!(U15, u16, 15);
uint!(U22, u32, 22);

pub trait Bits {
    fn get_bits(self, high: u32, low: u32) -> Self;

    fn set_bits(&mut self, src: Self, high: u32, low: u32);
}

macro_rules! bits_impl {
    ($u:ident) => {
        impl Bits for $u {
            #[inline]
            fn get_bits(self, high: u32, low: u32) -> $u {
                assert!(high >= low);
                if low > $u::WIDTH {
                    0
                } else {
                    (self >> low).mod_power_of_two(u64::from(high - low + 1))
                }
            }

            fn set_bits(&mut self, src: Self, high: u32, low: u32) {
                let mask = (1 << (high + 1)) - (1 << low);
                *self = (*self & !mask) | ((src << low) & mask);
            }
        }
    };
}

bits_impl!(u8);
bits_impl!(u16);
bits_impl!(u32);
bits_impl!(u64);
