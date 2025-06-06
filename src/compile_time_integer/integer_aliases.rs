// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
//Make sure to never let rustfmt touch this file. There's an attribute in the super module which
//unfortunately can't be here for some reason.
//!Type aliases for compile-time integers from -10 to 10.
#![allow(missing_docs)]
use super::*;
pub use super::Zero;
pub type Neg10 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>>>>>>;
pub type Neg9 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>>>>>;
pub type Neg8 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>>>>;
pub type Neg7 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>>>;
pub type Neg6 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>>;
pub type Neg5 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>>;
pub type Neg4 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>>;
pub type Neg3 = NegativeOnePlus<NegativeOnePlus<NegativeOnePlus<Zero>>>;
pub type Neg2 = NegativeOnePlus<NegativeOnePlus<Zero>>;
pub type Neg1 = NegativeOnePlus<Zero>;
pub type Pos1 = OnePlus<Zero>;
pub type Pos2 = OnePlus<OnePlus<Zero>>;
pub type Pos3 = OnePlus<OnePlus<OnePlus<Zero>>>;
pub type Pos4 = OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>;
pub type Pos5 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>;
pub type Pos6 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>>;
pub type Pos7 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>>>;
pub type Pos8 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>>>>;
pub type Pos9 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>>>>>;
pub type Pos10 = OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<OnePlus<Zero>>>>>>>>>>;
