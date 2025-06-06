// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//Make sure to never let rustfmt touch this file. There's an attribute in the super module which
//unfortunately can't be here for some reason.
//!Type aliases for compile-time integers from -10 to 10.
#![allow(missing_docs)]
use super::*;
pub use super::Zero;
pub type Neg1 = NegativeOnePlus<Zero>;
pub type Neg2 = NegativeOnePlus<Neg1>;
pub type Neg3 = NegativeOnePlus<Neg2>;
pub type Neg4 = NegativeOnePlus<Neg3>;
pub type Neg5 = NegativeOnePlus<Neg4>;
pub type Neg6 = NegativeOnePlus<Neg5>;
pub type Neg7 = NegativeOnePlus<Neg6>;
pub type Neg8 = NegativeOnePlus<Neg7>;
pub type Neg9 = NegativeOnePlus<Neg8>;
pub type Neg10 = NegativeOnePlus<Neg9>;
pub type Pos1 = OnePlus<Zero>;
pub type Pos2 = OnePlus<Pos1>;
pub type Pos3 = OnePlus<Pos2>;
pub type Pos4 = OnePlus<Pos3>;
pub type Pos5 = OnePlus<Pos4>;
pub type Pos6 = OnePlus<Pos5>;
pub type Pos7 = OnePlus<Pos6>;
pub type Pos8 = OnePlus<Pos7>;
pub type Pos9 = OnePlus<Pos8>;
pub type Pos10 = OnePlus<Pos9>;
