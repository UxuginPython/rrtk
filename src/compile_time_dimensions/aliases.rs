// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//Make sure to never let rustfmt touch this file. There's an attribute in the super module which
//unfortunately can't be here for some reason.
//!Type aliases for [`Quantity`] types of different dimensions.
#![allow(missing_docs)]
use compile_time_integer::aliases::*;
use super::*;
pub type InverseMillimeterCubedSecondCubed<T> = Quantity<T, Neg3, Neg3>;
pub type InverseMillimeterCubedSecondSquared<T> = Quantity<T, Neg3, Neg2>;
pub type InverseMillimeterCubedSecond<T> = Quantity<T, Neg3, Neg1>;
pub type InverseMillimeterCubed<T> = Quantity<T, Neg3, Zero>;
pub type SecondPerMillimeterCubed<T> = Quantity<T, Neg3, Pos1>;
pub type SecondSquaredPerMillimeterCubed<T> = Quantity<T, Neg3, Pos2>;
pub type SecondCubedPerMillimeterCubed<T> = Quantity<T, Neg3, Pos2>;

pub type InverseMillimeterSquaredSecondCubed<T> = Quantity<T, Neg2, Neg3>;
pub type InverseMillimeterSquaredSecondSquared<T> = Quantity<T, Neg2, Neg2>;
pub type InverseMillimeterSquaredSecond<T> = Quantity<T, Neg2, Neg1>;
pub type InverseMillimeterSquared<T> = Quantity<T, Neg2, Zero>;
pub type SecondPerMillimeterSquared<T> = Quantity<T, Neg2, Pos1>;
pub type SecondSquaredPerMillimeterSquared<T> = Quantity<T, Neg2, Pos2>;
pub type SecondCubedPerMillimeterSquared<T> = Quantity<T, Neg2, Pos2>;

pub type InverseMillimeterSecondCubed<T> = Quantity<T, Neg1, Neg3>;
pub type InverseMillimeterSecondSquared<T> = Quantity<T, Neg1, Neg2>;
pub type InverseMillimeterSecond<T> = Quantity<T, Neg1, Neg1>;
pub type InverseMillimeter<T> = Quantity<T, Neg1, Zero>;
pub type SecondPerMillimeter<T> = Quantity<T, Neg1, Pos1>;
pub type SecondSquaredPerMillimeter<T> = Quantity<T, Neg1, Pos2>;
pub type SecondCubedPerMillimeter<T> = Quantity<T, Neg1, Pos2>;

pub type InverseSecondCubed<T> = Quantity<T, Zero, Neg3>;
pub type InverseSecondSquared<T> = Quantity<T, Zero, Neg2>;
pub type InverseSecond<T> = Quantity<T, Zero, Neg1>;
pub type Dimensionless<T> = Quantity<T, Zero, Zero>;
pub type Second<T> = Quantity<T, Zero, Pos1>;
pub type SecondSquared<T> = Quantity<T, Zero, Pos2>;
pub type SecondCubed<T> = Quantity<T, Zero, Pos3>;

pub type MillimeterPerSecondCubed<T> = Quantity<T, Pos1, Neg3>;
pub type MillimeterPerSecondSquared<T> = Quantity<T, Pos1, Neg2>;
pub type MillimeterPerSecond<T> = Quantity<T, Pos1, Neg1>;
pub type Millimeter<T> = Quantity<T, Pos1, Zero>;
pub type MillimeterSecond<T> = Quantity<T, Pos1, Pos1>;
pub type MillimeterSecondSquared<T> = Quantity<T, Pos1, Pos2>;
pub type MillimeterSecondCubed<T> = Quantity<T, Pos1, Pos3>;

pub type MillimeterSquaredPerSecondCubed<T> = Quantity<T, Pos2, Neg3>;
pub type MillimeterSquaredPerSecondSquared<T> = Quantity<T, Pos2, Neg2>;
pub type MillimeterSquaredPerSecond<T> = Quantity<T, Pos2, Neg1>;
pub type MillimeterSquared<T> = Quantity<T, Pos2, Zero>;
pub type MillimeterSquaredSecond<T> = Quantity<T, Pos2, Pos1>;
pub type MillimeterSquaredSecondSquared<T> = Quantity<T, Pos2, Pos2>;
pub type MillimeterSquaredSecondCubed<T> = Quantity<T, Pos2, Pos3>;

pub type MillimeterCubedPerSecondCubed<T> = Quantity<T, Pos3, Neg3>;
pub type MillimeterCubedPerSecondSquared<T> = Quantity<T, Pos3, Neg2>;
pub type MillimeterCubedPerSecond<T> = Quantity<T, Pos3, Neg1>;
pub type MillimeterCubed<T> = Quantity<T, Pos3, Zero>;
pub type MillimeterCubedSecond<T> = Quantity<T, Pos3, Pos1>;
pub type MillimeterCubedSecondSquared<T> = Quantity<T, Pos3, Pos2>;
pub type MillimeterCubedSecondCubed<T> = Quantity<T, Pos3, Pos3>;
