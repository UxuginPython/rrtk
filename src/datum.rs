// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2026 UxuginPython
use crate::*;
use stulta::NotDatum;
///Holds a timestamp and something else, usually an `f32`, a [`Quantity`], or one of the state
///types.
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
        Datum { time, value }
    }
    //It *might* be possible to make this const fn with a little unsafe code similar to that in
    //Quantity::into_inner.
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
impl<T: Add<TR, Output = O>, TR, O> Add<Datum<TR>> for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: Datum<TR>) -> Datum<O> {
        let output_value = self.value + other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: AddAssign<TR>, TR> AddAssign<Datum<TR>> for Datum<T> {
    fn add_assign(&mut self, other: Datum<TR>) {
        self.value += other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Add<TR, Output = O>, TR: NotDatum, O> Add<TR> for Datum<T> {
    type Output = Datum<O>;
    fn add(self, other: TR) -> Datum<O> {
        let output_value = self.value + other;
        Datum::new(self.time, output_value)
    }
}
impl<T: AddAssign<TR>, TR: NotDatum> AddAssign<TR> for Datum<T> {
    fn add_assign(&mut self, other: TR) {
        self.value += other;
    }
}
impl<T: Sub<TR, Output = O>, TR, O> Sub<Datum<TR>> for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: Datum<TR>) -> Datum<O> {
        let output_value = self.value - other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: SubAssign<TR>, TR> SubAssign<Datum<TR>> for Datum<T> {
    fn sub_assign(&mut self, other: Datum<TR>) {
        self.value -= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Sub<TR, Output = O>, TR: NotDatum, O> Sub<TR> for Datum<T> {
    type Output = Datum<O>;
    fn sub(self, other: TR) -> Datum<O> {
        let output_value = self.value - other;
        Datum::new(self.time, output_value)
    }
}
impl<T: SubAssign<TR>, TR: NotDatum> SubAssign<TR> for Datum<T> {
    fn sub_assign(&mut self, other: TR) {
        self.value -= other;
    }
}
impl<T: Mul<TR, Output = O>, TR, O> Mul<Datum<TR>> for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: Datum<TR>) -> Datum<O> {
        let output_value = self.value * other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: MulAssign<TR>, TR> MulAssign<Datum<TR>> for Datum<T> {
    fn mul_assign(&mut self, other: Datum<TR>) {
        self.value *= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Mul<TR, Output = O>, TR: NotDatum, O> Mul<TR> for Datum<T> {
    type Output = Datum<O>;
    fn mul(self, other: TR) -> Datum<O> {
        let output_value = self.value * other;
        Datum::new(self.time, output_value)
    }
}
impl<T: MulAssign<TR>, TR: NotDatum> MulAssign<TR> for Datum<T> {
    fn mul_assign(&mut self, other: TR) {
        self.value *= other;
    }
}
impl<T: Div<TR, Output = O>, TR, O> Div<Datum<TR>> for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: Datum<TR>) -> Datum<O> {
        let output_value = self.value / other.value;
        let output_time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
        Datum::new(output_time, output_value)
    }
}
impl<T: DivAssign<TR>, TR> DivAssign<Datum<TR>> for Datum<T> {
    fn div_assign(&mut self, other: Datum<TR>) {
        self.value /= other.value;
        self.time = if self.time >= other.time {
            self.time
        } else {
            other.time
        };
    }
}
impl<T: Div<TR, Output = O>, TR: NotDatum, O> Div<TR> for Datum<T> {
    type Output = Datum<O>;
    fn div(self, other: TR) -> Datum<O> {
        let output_value = self.value / other;
        Datum::new(self.time, output_value)
    }
}
impl<T: DivAssign<TR>, TR: NotDatum> DivAssign<TR> for Datum<T> {
    fn div_assign(&mut self, other: TR) {
        self.value /= other;
    }
}
