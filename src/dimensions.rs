// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
//!RRTK's compile-time dimensional analysis system. This system is simpler than ones like
//![`uom`](https://crates.io/crates/uom), but it serves a similar purpose: to protect users from
//!dimension mismatch errors at compile time without runtime overhead.
//!
//!This is done through a
//![semi-hack](compile_time_integer) representing integers as types and adding type parameters to a
//!special struct called [`Quantity`], which is a transparent struct holding only a value at
//!runtime. There are also a few other specialized types for values that are better represented
//!with integers than floating point numbers but still must interact with floating point values.
//!
//!# Unit safety
//!The dimensional analysis system uses a concept called *unit safety* to explain how dimensional
//!analysis is handled in various operations. Although unit safety can be mentally modeled in a
//!similar way to memory safety, the two are unrelated technically. Memory safety or unsafety should
//!not be taken to imply unit safety or unsafety, and unit safety or unsafety should not be taken to
//!imply memory safety or unsafety.
//!
//!To understand unit safety, one must first understand *unit correctness*. For code to be
//!*unit-correct*, no numerical value it has may be *marked* with an incorrect unit.
//!## Marked units
//!There are a few ways for a value to be *marked* with a unit. Here are a few:
//!- Values stored in `Quantity` are marked with the unit specified by the unit parameters.
//!- `DimensionlessInteger` and `DimensionlessFraction` are marked as dimensionless.
//!- The argument to [`Time::from_nanoseconds`] is marked as nanoseconds.
//!- The argument to [`Time::from_seconds_f32`] is marked as seconds.
use super::*;
use compile_time_integer::*;
use core::num::NonZero;
//This attribute currently cannot be in the actual file with #![].
#[rustfmt::skip]
pub mod dimension_aliases;
pub use dimension_aliases::*;
pub mod transmute_safe {
    use super::*;
    pub trait UnitMarker {}
    pub mod unit_markers {
        use super::*;
        pub struct MillimeterSecond<MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>);
        impl<MM: Integer, S: Integer> UnitMarker for MillimeterSecond<MM, S> {}
        #[non_exhaustive]
        pub struct Nanosecond;
        impl UnitMarker for Nanosecond {}
    }
    pub trait CanRepresent<U: UnitMarker> {}
    impl<T, MM: Integer, S: Integer> CanRepresent<unit_markers::MillimeterSecond<MM, S>>
        for Quantity<T, MM, S>
    {
    }
    impl CanRepresent<unit_markers::MillimeterSecond<Zero, Zero>> for DimensionlessInteger {}
    impl CanRepresent<unit_markers::MillimeterSecond<Zero, Zero>> for DimensionlessFraction {}
    impl CanRepresent<unit_markers::Nanosecond> for Time {}
    macro_rules! impl_all_can_represent {
        ($num_type: ty, $($other_impls: ty),+) => {
            impl<U: UnitMarker> CanRepresent<U> for $num_type {}
            impl_all_can_represent!($($other_impls),+);
        };
        ($num_type: ty) => {
            impl<U: UnitMarker> CanRepresent<U> for $num_type {}
        };
    }
    impl_all_can_represent!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
    );
    pub trait OnlyRepresents: CanRepresent<Self::Unit> {
        type Unit: UnitMarker;
    }
    impl<T, MM: Integer, S: Integer> OnlyRepresents for Quantity<T, MM, S> {
        type Unit = unit_markers::MillimeterSecond<MM, S>;
    }
    impl OnlyRepresents for DimensionlessInteger {
        type Unit = unit_markers::MillimeterSecond<Zero, Zero>;
    }
    impl OnlyRepresents for DimensionlessFraction {
        type Unit = unit_markers::MillimeterSecond<Zero, Zero>;
    }
    impl OnlyRepresents for Time {
        type Unit = unit_markers::Nanosecond;
    }
    pub unsafe trait Transparent {
        type Inner;
    }
    unsafe impl<T, MM: Integer, S: Integer> Transparent for Quantity<T, MM, S> {
        type Inner = T;
    }
    unsafe impl Transparent for DimensionlessInteger {
        type Inner = i64;
    }
    unsafe impl Transparent for Time {
        type Inner = i64;
    }
    macro_rules! impl_all_transparent {
        ($num_type: ty, $($other_impls: ty),+) => {
            unsafe impl Transparent for $num_type {
                type Inner = Self;
            }
            impl_all_transparent!($($other_impls),+);
        };
        ($num_type: ty) => {
            unsafe impl Transparent for $num_type {
                type Inner = Self;
            }
        };
    }
    impl_all_transparent!(
        u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
    );
    const unsafe fn force_transmute<Src: Copy, Dst: Copy>(src: Src) -> Dst {
        #[repr(C)]
        union Transmute<A: Copy, B: Copy> {
            src: A,
            dst: B,
        }
        let transmute = Transmute { src };
        unsafe { transmute.dst }
    }
    #[inline(always)]
    pub const fn transmute_memory_safe<A, B>(was: A) -> B
    where
        A: Transparent + Copy,
        B: Transparent<Inner = A::Inner> + Copy,
    {
        unsafe { force_transmute(was) }
    }
    #[inline(always)]
    pub const fn transmute_unit_safe<A, B>(was: A) -> B
    where
        A: Transparent + Copy + OnlyRepresents,
        B: Transparent<Inner = A::Inner> + Copy + CanRepresent<A::Unit>,
    {
        transmute_memory_safe(was)
    }
}
///A time stored internally in `i64` nanoseconds.
///
///`Time` is often converted to [`Second<f32>`] to interact with quantities of other dimensions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Time(i64);
impl Time {
    ///Zero time. You would get this from `Time::from_nanoseconds(0)`.
    pub const ZERO: Self = Time(0);
    ///Construct a `Time` from `i64` nanoseconds, which is how the time is stored internally.
    pub const fn from_nanoseconds(value: i64) -> Self {
        Self(value)
    }
    ///Construct a `Time` from `f32` seconds.
    pub const fn from_seconds_f32(value: f32) -> Self {
        Self((value * 1_000_000_000.0) as i64)
    }
    ///Construct a `Time` from [`Quantity`] seconds stored using `f32`.
    pub fn from_seconds(value: Second<f32>) -> Self {
        Self::from_seconds_f32(value.into_inner())
    }
    ///Get the internal `i64` nanoseconds from the `Time`.
    pub const fn as_nanoseconds(self) -> i64 {
        self.0
    }
    ///Get the value of the `Time` as `f32` seconds.
    pub const fn as_seconds_f32(self) -> f32 {
        (self.0 as f32) / 1_000_000_000.0
    }
    ///Get the value of the `Time` as [`Quantity`] seconds stored using `f32`.
    ///Effectively a wrapper for [`as_seconds`](Self::as_seconds).
    pub const fn as_seconds(self) -> Second<f32> {
        Second::new(self.as_seconds_f32())
    }
}
impl From<Second<f32>> for Time {
    fn from(was: Second<f32>) -> Self {
        Self::from_seconds(was)
    }
}
impl From<Time> for Second<f32> {
    fn from(was: Time) -> Self {
        was.as_seconds()
    }
}
impl Add for Time {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for Time {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Neg for Time {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<DimensionlessInteger> for Time {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign<DimensionlessInteger> for Time {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        self.0 *= rhs.0;
    }
}
impl Div<DimensionlessInteger> for Time {
    type Output = Self;
    fn div(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 / rhs.0)
    }
}
impl DivAssign<DimensionlessInteger> for Time {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        self.0 /= rhs.0;
    }
}
///Converts the time to `f32` seconds before the operation. This is to make `f32` compatible with
///[`streams::math::IntegralStream`].
impl Mul<f32> for Time {
    type Output = f32;
    fn mul(self, rhs: f32) -> f32 {
        self.as_seconds_f32() * rhs
    }
}
///Converts the time to `f32` seconds before the operation.
impl Mul<Time> for f32 {
    type Output = Self;
    fn mul(self, rhs: Time) -> Self {
        self * rhs.as_seconds_f32()
    }
}
///Converts the time to `f32` seconds before the operation.
impl Div<f32> for Time {
    type Output = f32;
    fn div(self, rhs: f32) -> f32 {
        self.as_seconds_f32() / rhs
    }
}
///Converts the time to `f32` seconds before the operation. This is to make `f32` compatible with
///[`streams::math::DerivativeStream`].
impl Div<Time> for f32 {
    type Output = Self;
    fn div(self, rhs: Time) -> Self {
        self / rhs.as_seconds_f32()
    }
}
///A dimensionless value stored as an integer. Used almost exclusively for when a time, stored
///as an integer, must be multiplied by a constant factor as in numerical integrals and motion
///profiles.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct DimensionlessInteger(pub i64);
impl DimensionlessInteger {
    ///Constructor for [`DimensionlessInteger`].
    #[inline(always)]
    pub const fn new(value: i64) -> Self {
        Self(value)
    }
    ///`x.const_eq(y)` is exactly equivalent to `x == y` except that it works in const contexts.
    #[inline(always)]
    pub const fn const_eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
    ///Checks if the integer is zero.
    #[inline(always)]
    pub const fn is_zero(&self) -> bool {
        self.0 == 0
    }
    ///Converts from `DimensionlessInteger` to `Quantity<i64, Zero, Zero>`.
    ///
    ///The following two lines are guaranteed to have the same effect given `DimensionlessInteger`
    ///variable `x`:
    ///```
    ///# use rrtk::*;
    ///# let x = DimensionlessInteger(4);
    ///let y = x.as_quantity();
    ///# assert_eq!(y, dimensions::layout_compatibility::safe_transmute(x));
    ///```
    ///```
    ///# use rrtk::*;
    ///# use rrtk::compile_time_integer::Zero;
    ///# let x = DimensionlessInteger(4);
    ///let y: Quantity<i64, Zero, Zero> = dimensions::layout_compatibility::safe_transmute(x);
    ///# assert_eq!(y, x.as_quantity());
    ///```
    #[inline(always)]
    pub const fn as_quantity(self) -> Quantity<i64, Zero, Zero> {
        Quantity::new(self.0)
    }
}
impl From<i64> for DimensionlessInteger {
    fn from(was: i64) -> Self {
        Self(was)
    }
}
impl From<DimensionlessInteger> for i64 {
    fn from(was: DimensionlessInteger) -> Self {
        was.0
    }
}
impl Add for DimensionlessInteger {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for DimensionlessInteger {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Sub for DimensionlessInteger {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}
impl SubAssign for DimensionlessInteger {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}
impl Mul for DimensionlessInteger {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}
impl MulAssign for DimensionlessInteger {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}
impl Div for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn div(self, rhs: Self) -> DimensionlessFraction {
        DimensionlessFraction::new(self, rhs)
    }
}
impl Neg for DimensionlessInteger {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
impl Mul<Time> for DimensionlessInteger {
    type Output = Time;
    fn mul(self, rhs: Time) -> Time {
        Time(self.0 * rhs.0)
    }
}
///An exact rational number type for dimensionless values.
///
///There is a memory safety guarantee that the denominator is nonzero. This means that undefined
///behavior immediately occurs if an instance of this type exists with a zero denominator,
///regardless of whether the instance is used in any way.
#[derive(Clone, Copy, Debug)]
pub struct DimensionlessFraction(i64, NonZero<i64>);
impl DimensionlessFraction {
    ///Tries to check whether the denominator is zero and panic if it is.
    ///
    ///As long as the denominator is nonzero, this method is guaranteed to have no effect.
    ///Importantly, however, if the denominator *is* zero, undefined behavior has already begun, and
    ///this method cannot do anything about it. It will still try to panic, but nothing is
    ///guaranteed.
    #[inline]
    pub const fn assert_valid(&self) {
        assert!(
            self.1.get() != 0,
            "DimensionlessFraction with zero denominator detected - this indicates undefined behavior"
        );
    }
    ///With debug assertions enabled, identical to [`assert_valid`](Self::assert_valid). With debug
    ///assertions disabled (typically in release mode), NOP.
    #[inline]
    pub const fn debug_assert_valid(&self) {
        debug_assert!(
            self.1.get() != 0,
            "DimensionlessFraction with zero denominator detected - this indicates undefined behavior"
        );
    }
    ///Constructor that panics if the provided denominator is zero.
    #[inline]
    pub const fn new<N, D>(num: N, denom: D) -> Self
    where
        N: transmute_safe::Transparent<Inner = i64> + transmute_safe::OnlyRepresents + Copy,
        D: transmute_safe::Transparent<Inner = i64>
            + transmute_safe::OnlyRepresents<Unit = N::Unit>
            + Copy,
    {
        let denom = NonZero::new(transmute_safe::transmute_unit_safe(denom))
            .expect("tried to construct DimensionlessFraction with zero denominator");
        Self(transmute_safe::transmute_unit_safe(num), denom)
    }
    ///Constructor that does **not** verify that the denominator is nonzero.
    ///
    ///Calling this function with a denominator of zero is undefined behavior.
    ///However, with debug assertions enabled, this will still perform the nonzero denominator
    ///assertion.
    #[inline(always)]
    pub const unsafe fn new_unchecked<N, D>(num: N, denom: D) -> Self
    where
        N: transmute_safe::Transparent<Inner = i64> + transmute_safe::OnlyRepresents + Copy,
        D: transmute_safe::Transparent<Inner = i64>
            + transmute_safe::OnlyRepresents<Unit = N::Unit>
            + Copy,
    {
        if cfg!(debug_assertions) {
            Self::new(num, denom)
        } else {
            Self(transmute_safe::transmute_unit_safe(num), unsafe {
                NonZero::new_unchecked(transmute_safe::transmute_unit_safe(denom))
            })
        }
    }
    ///Constructor from raw `i64` values for numerator and denominator that panics if the provided
    ///denominator is zero.
    #[inline]
    pub const fn from_raw(num: i64, denom: i64) -> Self {
        Self(
            num,
            NonZero::new(denom)
                .expect("tried to construct DimensionlessFraction with zero denominator"),
        )
    }
    //FIXME: It seems inconsistent to sometimes have the check in debug mode anyway and sometimes
    //not.
    ///Constructor from raw `i64` values for numerator and denominator that does **not** verify that
    ///the denominator is nonzero.
    ///
    ///Calling this function with a denominator of zero is undefined behavior. This function does
    ///not perform the nonzero denominator assertion, even with debug assertions enabled.
    #[inline]
    pub const unsafe fn from_raw_unchecked(num: i64, denom: i64) -> Self {
        Self(num, unsafe { NonZero::new_unchecked(denom) })
    }
    ///Constructor from an `i64` numerator and a `NonZero<i64>` denominator.
    ///
    ///The numerator and denominator are internally stored by `DimensionlessFraction` as these
    ///types, so this is the most efficient constructor. As for safety, it is the caller's
    ///responsibility to make sure that the `NonZero` denominator is valid.
    #[inline(always)]
    pub const fn from_true_raw(num: i64, denom: NonZero<i64>) -> Self {
        Self(num, denom)
    }
    ///Reciprocal function (1/x) that panics if the new denominator is zero.
    #[inline]
    pub const fn reciprocal(&self) -> Self {
        Self::from_raw(self.1.get(), self.0)
    }
    ///Reciprocal function (1/x) that does **not** verify that the new denominator is nonzero.
    ///
    ///Calling this function on a fraction equal to 0 is undefined behavior.
    ///However, with debug assertions enabled, this will still perform the nonzero denominator
    ///assertion.
    #[inline(always)]
    pub const unsafe fn reciprocal_unchecked(&self) -> Self {
        if cfg!(debug_assertions) {
            self.reciprocal()
        } else {
            unsafe { Self::from_raw_unchecked(self.1.get(), self.0) }
        }
    }
    ///Converts the fraction into a tuple `(numerator, denominator)`.
    ///
    ///The following code is guaranteed to leave mutable [`DimensionlessInteger`] variables `x` and
    ///`y` with the same values that they had before the code was run as long as `y` is nonzero.
    ///```
    ///# use rrtk::{DimensionlessFraction, DimensionlessInteger};
    ///# let mut x = DimensionlessInteger(2);
    ///# let mut y = DimensionlessInteger(3);
    ///let frac = DimensionlessFraction::new(x, y);
    ///(x, y) = frac.into_components();
    ///# assert_eq!(x.0, 2);
    ///# assert_eq!(y.0, 3);
    ///```
    #[inline]
    pub const fn into_components(self) -> (DimensionlessInteger, DimensionlessInteger) {
        (
            DimensionlessInteger(self.0),
            DimensionlessInteger(self.1.get()),
        )
    }
    ///Converts the fraction into a tuple `(numerator, denominator)`.
    ///
    ///Unlike [`into_components`](Self::into_components), this method returns `(i64, NonZero<i64>)`,
    ///which matches the internal representations of the numerator and denominator.
    ///
    ///The following code is guaranteed to leave mutable `i64` variable `x` and mutable
    ///[`NonZero<i64>`] variable `y` with the same values that they had before the code was run.
    ///```
    ///# use rrtk::DimensionlessFraction;
    ///# let mut x = 2_i64;
    ///# let mut y = core::num::NonZero::new(3_i64).expect("literal value 3 is not 0");
    ///let frac = DimensionlessFraction::from_true_raw(x, y);
    ///(x, y) = frac.into_true_components();
    ///# assert_eq!(x, 2);
    ///# assert_eq!(y.get(), 3);
    ///```
    #[inline(always)]
    pub const fn into_true_components(self) -> (i64, NonZero<i64>) {
        (self.0, self.1)
    }
    ///Converts the fraction to its closest `f32` approximation.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_f32(&self) -> f32 {
        self.0 as f32 / self.1.get() as f32
    }
    ///Converts the fraction to its closest `f64` approximation.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_f64(&self) -> f64 {
        self.0 as f64 / self.1.get() as f64
    }
    ///Wraps the output of [`as_f32`](Self::as_f32) in a `Dimensionless` wrapper.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_quantity_f32(&self) -> Dimensionless<f32> {
        Dimensionless::new(self.as_f32())
    }
    ///Wraps the output of [`as_f64`](Self::as_f64) in a `Dimensionless` wrapper.
    ///There is also a [`From`] implementation that does this.
    #[inline]
    pub const fn as_quantity_f64(&self) -> Dimensionless<f64> {
        Dimensionless::new(self.as_f64())
    }
    ///Returns true if the numerators and denominators are directly equal. For example, for
    ///fractions a/b and c/d, the `PartialEq` implementation tests for whether a/b=c/d, but this
    ///method tests whether a=c and b=d.
    #[inline]
    pub const fn raw_eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0 && self.1.get() == rhs.1.get()
    }
}
///The default `DimensionlessFraction` is 0, specifically 0/1, to match the other Rust numeric
///types.
impl Default for DimensionlessFraction {
    #[inline(always)]
    fn default() -> Self {
        const { Self::from_raw(0, 1) }
    }
}
impl From<DimensionlessInteger> for DimensionlessFraction {
    fn from(was: DimensionlessInteger) -> Self {
        Self(
            was.0,
            const { NonZero::new(1).expect("literal 1 is not 0") },
        )
    }
}
impl Ord for DimensionlessFraction {
    fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
        let a = self.0 * rhs.1.get();
        let b = self.1.get() * rhs.0;
        let cmp = a.cmp(&b);
        //This is true if the signs of the denominators match.
        //This assumes that neither denominator is zero.
        if (self.1.get() < 0) == (rhs.1.get() < 0) {
            cmp
        } else {
            cmp.reverse()
        }
    }
}
impl PartialEq for DimensionlessFraction {
    fn eq(&self, rhs: &Self) -> bool {
        self.cmp(rhs) == core::cmp::Ordering::Equal
    }
}
impl Eq for DimensionlessFraction {}
impl PartialOrd for DimensionlessFraction {
    fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}
impl Neg for DimensionlessFraction {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0, self.1)
    }
}
//This change is not considered breaking because The Book says "Relying on integer overflow’s
//wrapping behavior is considered an error."
//https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow
///This implementation panics if the denominator multiplication overflows, even in release mode.
impl Mul for DimensionlessFraction {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0,
            self.1
                .checked_mul(rhs.1)
                .expect("denominator overflow when multiplying DimensionlessFractions"),
        )
    }
}
impl MulAssign for DimensionlessFraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for DimensionlessFraction {
    type Output = Self;
    #[expect(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self {
        self * rhs.reciprocal()
    }
}
impl DivAssign for DimensionlessFraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Add for DimensionlessFraction {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_raw(
            self.0 * rhs.1.get() + rhs.0 * self.1.get(),
            self.1.get() * rhs.1.get(),
        )
    }
}
impl AddAssign for DimensionlessFraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for DimensionlessFraction {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}
impl SubAssign for DimensionlessFraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn mul(self, rhs: DimensionlessInteger) -> Self {
        Self(self.0 * rhs.0, self.1)
    }
}
impl MulAssign<DimensionlessInteger> for DimensionlessFraction {
    fn mul_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self * rhs;
    }
}
impl Div<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    #[expect(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: DimensionlessInteger) -> Self {
        Self::from_raw(self.0, self.1.get() * rhs.0)
    }
}
impl DivAssign<DimensionlessInteger> for DimensionlessFraction {
    fn div_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self / rhs;
    }
}
impl Add<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn add(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(rhs)
    }
}
impl AddAssign<DimensionlessInteger> for DimensionlessFraction {
    fn add_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self + rhs;
    }
}
impl Sub<DimensionlessInteger> for DimensionlessFraction {
    type Output = Self;
    fn sub(self, rhs: DimensionlessInteger) -> Self {
        self + Self::from(-rhs)
    }
}
impl SubAssign<DimensionlessInteger> for DimensionlessFraction {
    fn sub_assign(&mut self, rhs: DimensionlessInteger) {
        *self = *self - rhs;
    }
}
impl Mul<Time> for DimensionlessFraction {
    type Output = Time;
    fn mul(self, rhs: Time) -> Time {
        Time::from_nanoseconds(rhs.as_nanoseconds() * self.0 / self.1.get())
    }
}
impl Mul<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn mul(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        rhs * self
    }
}
impl Div<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        self * rhs.reciprocal()
    }
}
impl Mul<DimensionlessFraction> for Time {
    type Output = Self;
    fn mul(self, rhs: DimensionlessFraction) -> Self {
        rhs * self
    }
}
impl MulAssign<DimensionlessFraction> for Time {
    fn mul_assign(&mut self, rhs: DimensionlessFraction) {
        *self = *self * rhs;
    }
}
impl Div<DimensionlessFraction> for Time {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: DimensionlessFraction) -> Self {
        self * rhs.reciprocal()
    }
}
impl DivAssign<DimensionlessFraction> for Time {
    fn div_assign(&mut self, rhs: DimensionlessFraction) {
        *self = *self / rhs;
    }
}
impl Add<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn add(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        rhs + self
    }
}
impl Sub<DimensionlessFraction> for DimensionlessInteger {
    type Output = DimensionlessFraction;
    fn sub(self, rhs: DimensionlessFraction) -> DimensionlessFraction {
        DimensionlessFraction::from(self) - rhs
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for f32 {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_f32()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for f64 {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_f64()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for Dimensionless<f32> {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_quantity_f32()
    }
}
///This conversion is not lossless.
impl From<DimensionlessFraction> for Dimensionless<f64> {
    fn from(was: DimensionlessFraction) -> Self {
        was.as_quantity_f64()
    }
}
///Gets the resulting type from multiplying values of two types. (Alias for
///`<$a as Mul<$b>>::Output`.)
///
///This is an important thing to be able to do when writing code that is
///generic over units as, since quantities of different units are different types, the
///fully qualified syntax gets unwieldy quickly when performing multiplication and division.
///
///You should be able to use `rrtk::mul!` and `rrtk::dimensions::mul!` interchangably.
///They are only listed separately due to Rust's special scoping rules for macros that are
///different from those for other items.
#[macro_export]
macro_rules! mul {
    ($a: ty, $b: ty) => {
        <$a as Mul<$b>>::Output
    };
}
pub use mul;
///Gets the resulting type from dividing values of two types. (Alias for
///`<$a as Div<$b>>::Output`.)
///
///This is an important thing to be able to do when writing code that is
///generic over units as, since quantities of different units are different types, the
///fully qualified syntax gets unwieldy quickly when performing multiplication and division.
///
///You should be able to use `rrtk::div!` and `rrtk::dimensions::div!` interchangably.
///They are only listed separately due to Rust's special scoping rules for macros that are
///different from those for other items.
#[macro_export]
macro_rules! div {
    ($a: ty, $b: ty) => {
        <$a as Div<$b>>::Output
    };
}
pub use div;
///A quantity with a unit. Dimensional analysis is performed at compile time through the type
///parameters' representations of unit exponents.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Quantity<T, MM: Integer, S: Integer>(PhantomData<MM>, PhantomData<S>, pub(crate) T);
impl<T, MM: Integer, S: Integer> Quantity<T, MM, S> {
    ///Constructor for `Quantity`.
    #[inline]
    pub const fn new(inner: T) -> Self {
        Self(PhantomData, PhantomData, inner)
    }
    //This is not as simple as returning self.2 because of E0493 saying that Quantity's destructor
    //cannot be evaluated at compile-time. Quantity, however, has no Drop impl and is
    //#[repr(transparent)], so "drop glue" is unnecessary. This is the way of telling the compiler
    //that. Also, core::mem::transmute doesn't work because of the generic type.
    ///Converts the `Quantity` into its inner contained object, consuming it.
    #[inline]
    pub const fn into_inner(self) -> T {
        //XXX: This explicitly skips any Drop code for Quantity. It will probably have to stop
        //being const fn if Drop is ever implemented.
        use core::mem::ManuallyDrop;
        let x: ManuallyDrop<Self> = ManuallyDrop::new(self);
        let x_ptr: *const ManuallyDrop<Self> = &raw const x;
        let y_ptr: *const T = x_ptr.cast();
        unsafe { core::ptr::read(y_ptr) }
    }
}
impl Quantity<i64, Zero, Zero> {
    ///Converts from `Quantity<i64, Zero, Zero>` to `DimensionlessInteger`.
    ///
    ///The following two lines are guaranteed to have the same effect given
    ///`Quantity<i64, Zero, Zero>` variable `x`:
    ///```
    ///# use rrtk::*;
    ///# let x = Dimensionless::new(4_i64);
    ///let y = x.as_dimensionless_integer();
    ///# assert_eq!(y, dimensions::layout_compatibility::safe_transmute(x));
    ///```
    ///```
    ///# use rrtk::*;
    ///# let x = Dimensionless::new(4_i64);
    ///let y: DimensionlessInteger = dimensions::layout_compatibility::safe_transmute(x);
    ///# assert_eq!(y, x.as_dimensionless_integer());
    ///```
    #[inline(always)]
    pub const fn as_dimensionless_integer(self) -> DimensionlessInteger {
        DimensionlessInteger(self.2)
    }
}
macro_rules! impl_quantity_abs {
    ($t: ty) => {
        impl<MM: Integer, S: Integer> Quantity<$t, MM, S> {
            ///Evaluate the absolute value of the quantity.
            pub const fn abs(self) -> Self {
                Self(PhantomData, PhantomData, self.2.abs())
            }
        }
    };
}
impl_quantity_abs!(f32);
impl_quantity_abs!(f64);
impl_quantity_abs!(i8);
impl_quantity_abs!(i16);
impl_quantity_abs!(i32);
impl_quantity_abs!(i64);
impl_quantity_abs!(i128);
impl_quantity_abs!(isize);
impl<T, MM: Integer, S: Integer> From<T> for Quantity<T, MM, S> {
    fn from(was: T) -> Self {
        Self(PhantomData, PhantomData, was)
    }
}
macro_rules! impl_const_ops {
    ($t: ty) => {
        impl<MM: Integer, S: Integer> Quantity<$t, MM, S> {
            ///Exactly like `+` except that it works in a const context.
            #[inline]
            pub const fn add_const(self, rhs: Self) -> Self {
                Quantity::new(self.2 + rhs.2)
            }
            ///Exactly like `+` except that it works in a const context.
            #[inline]
            pub const fn sub_const(self, rhs: Self) -> Self {
                Quantity::new(self.2 - rhs.2)
            }
        }
    };
}
impl_const_ops!(f32);
impl_const_ops!(f64);
impl_const_ops!(u8);
impl_const_ops!(u16);
impl_const_ops!(u32);
impl_const_ops!(u64);
impl_const_ops!(u128);
impl_const_ops!(usize);
impl_const_ops!(i8);
impl_const_ops!(i16);
impl_const_ops!(i32);
impl_const_ops!(i64);
impl_const_ops!(i128);
impl_const_ops!(isize);
//FIXME: E0210
//There's a very similar commented out impl in lib.rs.
/*impl<T, MM: Integer, S: Integer> From<Quantity<T, MM, S>> for T {
    fn from(was: Quantity<T, MM, S>) -> T {
        was.2
    }
}*/
impl<T: Neg<Output = O>, O, MM: Integer, S: Integer> Neg for Quantity<T, MM, S> {
    type Output = Quantity<O, MM, S>;
    fn neg(self) -> Quantity<O, MM, S> {
        Quantity::new(-self.2)
    }
}
impl<T: Add<U, Output = O>, U, O, MM: Integer, S: Integer> Add<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    type Output = Quantity<O, MM, S>;
    fn add(self, rhs: Quantity<U, MM, S>) -> Quantity<O, MM, S> {
        Quantity::from(self.2 + rhs.2)
    }
}
impl<T: AddAssign<U>, U, MM: Integer, S: Integer> AddAssign<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    fn add_assign(&mut self, rhs: Quantity<U, MM, S>) {
        self.2 += rhs.2;
    }
}
impl<T: Sub<U, Output = O>, U, O, MM: Integer, S: Integer> Sub<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    type Output = Quantity<O, MM, S>;
    fn sub(self, rhs: Quantity<U, MM, S>) -> Quantity<O, MM, S> {
        Quantity::from(self.2 - rhs.2)
    }
}
impl<T: SubAssign<U>, U, MM: Integer, S: Integer> SubAssign<Quantity<U, MM, S>>
    for Quantity<T, MM, S>
{
    fn sub_assign(&mut self, rhs: Quantity<U, MM, S>) {
        self.2 -= rhs.2;
    }
}
impl<T: Mul<U, Output = O>, U, O, MM1: Integer, S1: Integer, MM2: Integer, S2: Integer>
    Mul<Quantity<U, MM2, S2>> for Quantity<T, MM1, S1>
{
    type Output = Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>>;
    fn mul(self, rhs: Quantity<U, MM2, S2>) -> Quantity<O, MM1::Plus<MM2>, S1::Plus<S2>> {
        Quantity::from(self.2 * rhs.2)
    }
}
impl<T: MulAssign<U>, U, MM: Integer, S: Integer> MulAssign<Dimensionless<U>>
    for Quantity<T, MM, S>
{
    fn mul_assign(&mut self, rhs: Dimensionless<U>) {
        self.2 *= rhs.2;
    }
}
impl<T: Div<U, Output = O>, U, O, MM1: Integer, S1: Integer, MM2: Integer, S2: Integer>
    Div<Quantity<U, MM2, S2>> for Quantity<T, MM1, S1>
{
    type Output = Quantity<O, MM1::Minus<MM2>, S1::Minus<S2>>;
    fn div(self, rhs: Quantity<U, MM2, S2>) -> Quantity<O, MM1::Minus<MM2>, S1::Minus<S2>> {
        Quantity::from(self.2 / rhs.2)
    }
}
impl<T: DivAssign<U>, U, MM: Integer, S: Integer> DivAssign<Dimensionless<U>>
    for Quantity<T, MM, S>
{
    fn div_assign(&mut self, rhs: Dimensionless<U>) {
        self.2 /= rhs.2;
    }
}
impl<MM: Integer, S: Integer> Mul<Time> for Quantity<f32, MM, S>
where
    //MM + 0 = MM
    MM: Integer<Plus<Zero> = MM>,
{
    type Output = Quantity<f32, MM, S::Plus<OnePlus<Zero>>>;
    fn mul(self, rhs: Time) -> Quantity<f32, MM, S::Plus<OnePlus<Zero>>> {
        self * rhs.as_seconds()
    }
}
impl<MM: Integer, S: Integer> Mul<Quantity<f32, MM, S>> for Time {
    type Output = Quantity<f32, MM, S::PlusOne>;
    fn mul(self, rhs: Quantity<f32, MM, S>) -> Quantity<f32, MM, S::PlusOne> {
        self.as_seconds() * rhs
    }
}
impl<MM: Integer, S: Integer> Div<Time> for Quantity<f32, MM, S>
where
    //MM - 0 = MM
    MM: Integer<Minus<Zero> = MM>,
{
    type Output = Quantity<f32, MM, S::Minus<OnePlus<Zero>>>;
    fn div(self, rhs: Time) -> Quantity<f32, MM, S::Minus<OnePlus<Zero>>> {
        self / rhs.as_seconds()
    }
}
impl<MM: Integer, S: Integer> Div<Quantity<f32, MM, S>> for Time
where
    MM: Integer<Negative = MM>,
{
    //S - 1 = -S + 1
    type Output = Quantity<f32, MM, <<S as Integer>::Negative as Integer>::PlusOne>;
    fn div(
        self,
        rhs: Quantity<f32, MM, S>,
    ) -> Quantity<f32, MM, <<S as Integer>::Negative as Integer>::PlusOne> {
        self.as_seconds() / rhs
    }
}
impl<T: fmt::Display, MM: Integer, S: Integer> fmt::Display for Quantity<T, MM, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} mm^{}s^{}", self.2, MM::as_i8(), S::as_i8())
    }
}
impl<T: stulta::Half, MM: Integer, S: Integer> stulta::Half for Quantity<T, MM, S> {
    fn half(self) -> Self {
        Self::new(self.2.half())
    }
}
impl<T: stulta::AbsoluteValue, MM: Integer, S: Integer> stulta::AbsoluteValue
    for Quantity<T, MM, S>
{
    fn rrtk_abs(self) -> Self {
        Self::new(self.2.rrtk_abs())
    }
}
//FIXME? It is a little weird that this just makes stuff a float when everything could in theory
//stay integer. It's just a lot easier to do one-off types for dimensionless and time quantities
//than it is to maintain a whole other side of the dimensional analysis system for exact values.
//Also, most of this is going to change in 0.8 or 0.9 anyway. Probably it will be changed to a
//system using optional external crates somehow and avoiding these kinds of special cases.
impl Div<Time> for DimensionlessFraction {
    type Output = InverseSecond<f32>;
    fn div(self, rhs: Time) -> InverseSecond<f32> {
        self.as_quantity_f32() / rhs
    }
}
