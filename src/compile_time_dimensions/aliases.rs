// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//Make sure to never let rustfmt touch this file. There's an attribute in the super module which
//unfortunately can't be here for some reason.
use super::*;
pub type InverseMillimeterCubedSecondCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type InverseMillimeterCubedSecondSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type InverseMillimeterCubedSecond<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, NegativeOnePlus<Zero>>;
pub type InverseMillimeterCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, Zero>;
pub type SecondPerMillimeterCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, OnePlus<Zero>>;
pub type SecondSquaredPerMillimeterCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, OnePlus<OnePlus<Zero>>>;
pub type SecondCubedPerMillimeterCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>, OnePlus<OnePlus<Zero>>>;

pub type InverseMillimeterSquaredSecondCubed<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type InverseMillimeterSquaredSecondSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type InverseMillimeterSquaredSecond<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, NegativeOnePlus<Zero>>;
pub type InverseMillimeterSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, Zero>;
pub type SecondPerMillimeterSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, OnePlus<Zero>>;
pub type SecondSquaredPerMillimeterSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, OnePlus<OnePlus<Zero>>>;
pub type SecondCubedPerMillimeterSquared<T> = Quantity<T, NegativeOnePlus<NegativeOnePlus<Zero>>, OnePlus<OnePlus<Zero>>>;

pub type InverseMillimeterSecondCubed<T> = Quantity<T, NegativeOnePlus<Zero>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type InverseMillimeterSecondSquared<T> = Quantity<T, NegativeOnePlus<Zero>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type InverseMillimeterSecond<T> = Quantity<T, NegativeOnePlus<Zero>, NegativeOnePlus<Zero>>;
pub type InverseMillimeter<T> = Quantity<T, NegativeOnePlus<Zero>, Zero>;
pub type SecondPerMillimeter<T> = Quantity<T, NegativeOnePlus<Zero>, OnePlus<Zero>>;
pub type SecondSquaredPerMillimeter<T> = Quantity<T, NegativeOnePlus<Zero>, OnePlus<OnePlus<Zero>>>;
pub type SecondCubedPerMillimeter<T> = Quantity<T, NegativeOnePlus<Zero>, OnePlus<OnePlus<Zero>>>;

pub type InverseSecondCubed<T> = Quantity<T, Zero, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type InverseSecondSquared<T> = Quantity<T, Zero, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type InverseSecond<T> = Quantity<T, Zero, NegativeOnePlus<Zero>>;
pub type Dimensionless<T> = Quantity<T, Zero, Zero>;
pub type Second<T> = Quantity<T, Zero, OnePlus<Zero>>;
pub type SecondSquared<T> = Quantity<T, Zero, OnePlus<OnePlus<Zero>>>;
pub type SecondCubed<T> = Quantity<T, Zero, OnePlus<OnePlus<OnePlus<Zero>>>>;

pub type MillimeterPerSecondCubed<T> = Quantity<T, OnePlus<Zero>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type MillimeterPerSecondSquared<T> = Quantity<T, OnePlus<Zero>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type MillimeterPerSecond<T> = Quantity<T, OnePlus<Zero>, NegativeOnePlus<Zero>>;
pub type Millimeter<T> = Quantity<T, OnePlus<Zero>, Zero>;
pub type MillimeterSecond<T> = Quantity<T, OnePlus<Zero>, OnePlus<Zero>>;
pub type MillimeterSecondSquared<T> = Quantity<T, OnePlus<Zero>, OnePlus<OnePlus<Zero>>>;
pub type MillimeterSecondCubed<T> = Quantity<T, OnePlus<Zero>, OnePlus<OnePlus<OnePlus<Zero>>>>;

pub type MillimeterSquaredPerSecondCubed<T> = Quantity<T, OnePlus<OnePlus<Zero>>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type MillimeterSquaredPerSecondSquared<T> = Quantity<T, OnePlus<OnePlus<Zero>>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type MillimeterSquaredPerSecond<T> = Quantity<T, OnePlus<OnePlus<Zero>>, NegativeOnePlus<Zero>>;
pub type MillimeterSquared<T> = Quantity<T, OnePlus<OnePlus<Zero>>, Zero>;
pub type MillimeterSquaredSecond<T> = Quantity<T, OnePlus<OnePlus<Zero>>, OnePlus<Zero>>;
pub type MillimeterSquaredSecondSquared<T> = Quantity<T, OnePlus<OnePlus<Zero>>, OnePlus<OnePlus<Zero>>>;
pub type MillimeterSquaredSecondCubed<T> = Quantity<T, OnePlus<OnePlus<Zero>>, OnePlus<OnePlus<OnePlus<Zero>>>>;

pub type MillimeterCubedPerSecondCubed<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type MillimeterCubedPerSecondSquared<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type MillimeterCubedPerSecond<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, NegativeOnePlus<Zero>>;
pub type MillimeterCubed<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, Zero>;
pub type MillimeterCubedSecond<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, OnePlus<Zero>>;
pub type MillimeterCubedSecondSquared<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, OnePlus<OnePlus<Zero>>>;
pub type MillimeterCubedSecondCubed<T> = Quantity<T, OnePlus<OnePlus<OnePlus<Zero>>>, OnePlus<OnePlus<OnePlus<Zero>>>>;
