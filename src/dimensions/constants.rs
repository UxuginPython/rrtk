//!Constant units. Millimeters are listed in the names before seconds except when second has a
//!positive exponent and millimeter a negative. Everything in this module is reexported both at the
//!`dimensions` module and at the crate level.
use crate::*;
pub const INVERSE_MILLIMETER_CUBED_SECOND_CUBED: Unit = Unit::new(-3, -3);
pub const INVERSE_MILLIMETER_CUBED_SECOND_SQUARED: Unit = Unit::new(-3, -2);
pub const INVERSE_MILLIMETER_CUBED_SECOND: Unit = Unit::new(-3, -1);
pub const INVERSE_MILLIMETER_CUBED: Unit = Unit::new(-3, 0);
pub const SECOND_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 1);
pub const SECOND_SQUARED_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 2);
pub const SECOND_CUBED_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 3);

pub const INVERSE_MILLIMETER_SQUARED_SECOND_CUBED: Unit = Unit::new(-2, -3);
pub const INVERSE_MILLIMETER_SQUARED_SECOND_SQUARED: Unit = Unit::new(-2, -2);
pub const INVERSE_MILLIMETER_SQUARED_SECOND: Unit = Unit::new(-2, -1);
pub const INVERSE_MILLIMETER_SQUARED: Unit = Unit::new(-2, 0);
pub const SECOND_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 1);
pub const SECOND_SQUARED_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 2);
pub const SECOND_CUBED_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 3);

pub const INVERSE_MILLIMETER_SECOND_CUBED: Unit = Unit::new(-1, -3);
pub const INVERSE_MILLIMETER_SECOND_SQUARED: Unit = Unit::new(-1, -2);
pub const INVERSE_MILLIMETER_SECOND: Unit = Unit::new(-1, -1);
pub const INVERSE_MILLIMETER: Unit = Unit::new(-1, 0);
pub const SECOND_PER_MILLIMETER: Unit = Unit::new(-1, 1);
pub const SECOND_SQUARED_PER_MILLIMETER: Unit = Unit::new(-1, 2);
pub const SECOND_CUBED_PER_MILLIMETER: Unit = Unit::new(-1, 3);

pub const INVERSE_SECOND_CUBED: Unit = Unit::new(0, -3);
pub const INVERSE_SECOND_SQUARED: Unit = Unit::new(0, -2);
pub const INVERSE_SECOND: Unit = Unit::new(0, -1);
pub const DIMENSIONLESS: Unit = Unit::new(0, 0);
pub const SECOND: Unit = Unit::new(0, 1);
pub const SECOND_SQUARED: Unit = Unit::new(0, 2);
pub const SECOND_CUBED: Unit = Unit::new(0, 3);

pub const MILLIMETER_PER_SECOND_CUBED: Unit = Unit::new(1, -3);
pub const MILLIMETER_PER_SECOND_SQUARED: Unit = Unit::new(1, -2);
pub const MILLIMETER_PER_SECOND: Unit = Unit::new(1, -1);
pub const MILLIMETER: Unit = Unit::new(1, 0);
pub const MILLIMETER_SECOND: Unit = Unit::new(1, 1);
pub const MILLIMETER_SECOND_SQUARED: Unit = Unit::new(1, 2);
pub const MILLIMETER_SECOND_CUBED: Unit = Unit::new(1, 3);

pub const MILLIMETER_SQUARED_PER_SECOND_CUBED: Unit = Unit::new(2, -3);
pub const MILLIMETER_SQUARED_PER_SECOND_SQUARED: Unit = Unit::new(2, -2);
pub const MILLIMETER_SQUARED_PER_SECOND: Unit = Unit::new(2, -1);
pub const MILLIMETER_SQUARED: Unit = Unit::new(2, 0);
pub const MILLIMETER_SQUARED_SECOND: Unit = Unit::new(2, 1);
pub const MILLIMETER_SQUARED_SECOND_SQUARED: Unit = Unit::new(2, 2);
pub const MILLIMETER_SQUARED_SECOND_CUBED: Unit = Unit::new(2, 3);

pub const MILLIMETER_CUBED_PER_SECOND_CUBED: Unit = Unit::new(3, -3);
pub const MILLIMETER_CUBED_PER_SECOND_SQUARED: Unit = Unit::new(3, -2);
pub const MILLIMETER_CUBED_PER_SECOND: Unit = Unit::new(3, -1);
pub const MILLIMETER_CUBED: Unit = Unit::new(3, 0);
pub const MILLIMETER_CUBED_SECOND: Unit = Unit::new(3, 1);
pub const MILLIMETER_CUBED_SECOND_SQUARED: Unit = Unit::new(3, 2);
pub const MILLIMETER_CUBED_SECOND_CUBED: Unit = Unit::new(3, 3);
