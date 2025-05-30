// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
use crate::*;
///A container for a time and something else, usually an [`f32`] or a [`State`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Datum<T> {
    ///Timestamp for the datum. This should probably be absolute.
    pub time: Time,
    ///The thing with the timestamp.
    pub value: T,
}
impl<T> Datum<T> {
    ///Constructor for [`Datum`] type.
    pub const fn new(time: Time, value: T) -> Datum<T> {
        Datum {
            time: time,
            value: value,
        }
    }
    ///Replaces `self` with `maybe_replace_with` if `maybe_replace_with`'s timestamp is newer than
    ///`self`'s. Returns true if `self` was replaced and false otherwise.
    pub fn replace_if_older_than(&mut self, maybe_replace_with: Self) -> bool {
        if maybe_replace_with.time > self.time {
            *self = maybe_replace_with;
            return true;
        }
        false
    }
}
///Extension trait for `Option<Datum<T>>`.
pub trait OptionDatumExt<T> {
    ///If `self` is `None`, replaces it with `Some(maybe_replace_with)`. If `self` is `Some`,
    ///replaces it with `Some(maybe_replace_with)` if `maybe_replace_with`'s timestamp is newer
    ///than its. Returns true if `self` was replaced and false otherwise.
    fn replace_if_none_or_older_than(&mut self, maybe_replace_with: Datum<T>) -> bool;
    ///If `maybe_replace_with` is `Some`, calls `replace_if_none_or_older_than`. If it is `None`,
    ///returns false immediately.
    fn replace_if_none_or_older_than_option(&mut self, maybe_replace_with: Self) -> bool;
}
impl<T> OptionDatumExt<T> for Option<Datum<T>> {
    fn replace_if_none_or_older_than(&mut self, maybe_replace_with: Datum<T>) -> bool {
        if let Some(self_datum) = self {
            if self_datum.time >= maybe_replace_with.time {
                return false;
            }
        }
        *self = Some(maybe_replace_with);
        true
    }
    fn replace_if_none_or_older_than_option(&mut self, maybe_replace_with: Self) -> bool {
        let maybe_replace_with = match maybe_replace_with {
            Some(x) => x,
            None => return false,
        };
        self.replace_if_none_or_older_than(maybe_replace_with)
    }
}
//Unfortunately implementing the ops traits is really awkward here and has unnecessary restrictions
//because of needing to provide implementations for T and Datum<T>. If we ever get negative trait
//bounds, it will be possible to provide a much better generic implementation for cases where the
//type of other is not necessarily the same as T. Additionally, I may just be doing it wrong.
//Regardless, for now, these are only implemented where other is T or Datum<T> and not a more
//generic type. Special cases for State and Command are provided due to their Mul<f32> and Div<f32>
//implementations.
impl<T: Not<Output = O>, O> Not for Datum<T> {
    type Output = Datum<O>;
    fn not(self) -> Datum<O> {
        Datum::new(self.time, !self.value)
    }
}
impl<T: Neg<Output = O>, O> Neg for Datum<T> {
    type Output = Datum<O>;
    fn neg(self) -> Datum<O> {
        Datum::new(self.time, -self.value)
    }
}
impl<T: Add<Output = O>, O> Add for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: Self) -> Datum<O> {
        let output_value = self.value + other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: AddAssign> AddAssign for Datum<T> {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Add<Output = O>, O> Add<T> for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: T) -> Datum<O> {
        let output_value = self.value + other;
        Datum::new(self.time, output_value)
    }
}
impl<T: AddAssign> AddAssign<T> for Datum<T> {
    fn add_assign(&mut self, other: T) {
        self.value += other;
    }
}
impl<T: Sub<Output = O>, O> Sub for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: Self) -> Datum<O> {
        let output_value = self.value - other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: SubAssign> SubAssign for Datum<T> {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Sub<Output = O>, O> Sub<T> for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: T) -> Datum<O> {
        let output_value = self.value - other;
        Datum::new(self.time, output_value)
    }
}
impl<T: SubAssign> SubAssign<T> for Datum<T> {
    fn sub_assign(&mut self, other: T) {
        self.value -= other;
    }
}
impl<T: Mul<Output = O>, O> Mul for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: Self) -> Datum<O> {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: MulAssign> MulAssign for Datum<T> {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Mul<Output = O>, O> Mul<T> for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: T) -> Datum<O> {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl<T: MulAssign> MulAssign<T> for Datum<T> {
    fn mul_assign(&mut self, other: T) {
        self.value *= other;
    }
}
impl<T: Div<Output = O>, O> Div for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: Self) -> Datum<O> {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: DivAssign> DivAssign for Datum<T> {
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Div<Output = O>, O> Div<T> for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: T) -> Datum<O> {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl<T: DivAssign> DivAssign<T> for Datum<T> {
    fn div_assign(&mut self, other: T) {
        self.value /= other;
    }
}
impl Mul<Datum<f32>> for Datum<State> {
    type Output = Self;
    fn mul(self, other: Datum<f32>) -> Self {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl MulAssign<Datum<f32>> for Datum<State> {
    fn mul_assign(&mut self, other: Datum<f32>) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Mul<f32> for Datum<State> {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl MulAssign<f32> for Datum<State> {
    fn mul_assign(&mut self, other: f32) {
        self.value *= other;
    }
}
impl Div<Datum<f32>> for Datum<State> {
    type Output = Self;
    fn div(self, other: Datum<f32>) -> Self {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl DivAssign<Datum<f32>> for Datum<State> {
    fn div_assign(&mut self, other: Datum<f32>) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Div<f32> for Datum<State> {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl DivAssign<f32> for Datum<State> {
    fn div_assign(&mut self, other: f32) {
        self.value /= other;
    }
}
impl Mul<Datum<f32>> for Datum<Command> {
    type Output = Self;
    fn mul(self, other: Datum<f32>) -> Self {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl MulAssign<Datum<f32>> for Datum<Command> {
    fn mul_assign(&mut self, other: Datum<f32>) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Mul<f32> for Datum<Command> {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl MulAssign<f32> for Datum<Command> {
    fn mul_assign(&mut self, other: f32) {
        self.value *= other;
    }
}
impl Div<Datum<f32>> for Datum<Command> {
    type Output = Self;
    fn div(self, other: Datum<f32>) -> Self {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl DivAssign<Datum<f32>> for Datum<Command> {
    fn div_assign(&mut self, other: Datum<f32>) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl Div<f32> for Datum<Command> {
    type Output = Self;
    fn div(self, other: f32) -> Self {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl DivAssign<f32> for Datum<Command> {
    fn div_assign(&mut self, other: f32) {
        self.value /= other;
    }
}
