//!Constant units. Millimeters are listed in the names before seconds except when second has a
//!positive exponent and millimeter a negative. Everything in this module is reexported both at the
//!`dimensions` module and at the crate level.
use crate::*;
///The `Unit` for a quantity in inverse millimeters cubed seconds cubed (mm^-3 s^-3).
pub const INVERSE_MILLIMETER_CUBED_SECOND_CUBED: Unit = Unit::new(-3, -3);
///The `Unit` for a quantity in inverse millimeters cubed seconds squared (mm^-3 s^-2).
pub const INVERSE_MILLIMETER_CUBED_SECOND_SQUARED: Unit = Unit::new(-3, -2);
///The `Unit` for a quantity in inverse millimeters cubed seconds (mm^-3 s^-1).
pub const INVERSE_MILLIMETER_CUBED_SECOND: Unit = Unit::new(-3, -1);
///The `Unit` for a quantity in inverse millimeters cubed (mm^-3).
pub const INVERSE_MILLIMETER_CUBED: Unit = Unit::new(-3, 0);
///The `Unit` for a quantity in seconds per millimeter cubed (s/mm^3).
pub const SECOND_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 1);
///The `Unit` for a quantity in seconds squared per millimeter cubed (s^2/mm^3).
pub const SECOND_SQUARED_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 2);
///The `Unit` for a quantity in seconds cubed per millimeter cubed (s^3/mm^3).
pub const SECOND_CUBED_PER_MILLIMETER_CUBED: Unit = Unit::new(-3, 3);

///The `Unit` for a quantity in inverse millimeters squared seconds cubed (mm^-2 s^-3).
pub const INVERSE_MILLIMETER_SQUARED_SECOND_CUBED: Unit = Unit::new(-2, -3);
///The `Unit` for a quantity in inverse millimeters squared seconds squared (mm^-2 s^-2).
pub const INVERSE_MILLIMETER_SQUARED_SECOND_SQUARED: Unit = Unit::new(-2, -2);
///The `Unit` for a quantity in inverse millimeters squared seconds (mm^-2 s^-1).
pub const INVERSE_MILLIMETER_SQUARED_SECOND: Unit = Unit::new(-2, -1);
///The `Unit` for a quantity in inverse millimeters squared (mm^-2).
pub const INVERSE_MILLIMETER_SQUARED: Unit = Unit::new(-2, 0);
///The `Unit` for a quantity in seconds per millimeter squared (s/mm^2).
pub const SECOND_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 1);
///The `Unit` for a quantity in seconds squared per millimeter squared (s^2/mm^2).
pub const SECOND_SQUARED_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 2);
///The `Unit` for a quantity in seconds cubed per millimeter squared (s^3/mm^2).
pub const SECOND_CUBED_PER_MILLIMETER_SQUARED: Unit = Unit::new(-2, 3);

///The `Unit` for a quantity in inverse millimeters seconds cubed (mm^-1 s^-3).
pub const INVERSE_MILLIMETER_SECOND_CUBED: Unit = Unit::new(-1, -3);
///The `Unit` for a quantity in inverse millimeters seconds squared (mm^-1 s^-2).
pub const INVERSE_MILLIMETER_SECOND_SQUARED: Unit = Unit::new(-1, -2);
///The `Unit` for a quantity in inverse millimeters seconds (mm^-1 s^-1).
pub const INVERSE_MILLIMETER_SECOND: Unit = Unit::new(-1, -1);
///The `Unit` for a quantity in inverse millimeters (mm^-1).
pub const INVERSE_MILLIMETER: Unit = Unit::new(-1, 0);
///The `Unit` for a quantity in seconds per millimeter (s/mm).
pub const SECOND_PER_MILLIMETER: Unit = Unit::new(-1, 1);
///The `Unit` for a quantity in seconds squared per millimeter (s^2/mm).
pub const SECOND_SQUARED_PER_MILLIMETER: Unit = Unit::new(-1, 2);
///The `Unit` for a quantity in seconds cubed per millimeter (s^3/mm).
pub const SECOND_CUBED_PER_MILLIMETER: Unit = Unit::new(-1, 3);

///The `Unit` for a quantity in inverse seconds cubed (s^-3).
pub const INVERSE_SECOND_CUBED: Unit = Unit::new(0, -3);
///The `Unit` for a quantity in inverse seconds squared (s^-2).
pub const INVERSE_SECOND_SQUARED: Unit = Unit::new(0, -2);
///The `Unit` for a quantity in inverse seconds (s^-1).
pub const INVERSE_SECOND: Unit = Unit::new(0, -1);
///The `Unit` for a dimensionless quantity.
pub const DIMENSIONLESS: Unit = Unit::new(0, 0);
///The `Unit` for a quantity in seconds (s).
pub const SECOND: Unit = Unit::new(0, 1);
///The `Unit` for a quantity in seconds squared (s^2).
pub const SECOND_SQUARED: Unit = Unit::new(0, 2);
///The `Unit` for a quantity in seconds cubed (s^3).
pub const SECOND_CUBED: Unit = Unit::new(0, 3);

///The `Unit` for a quantity in millimeters per second cubed (mm/s^3).
pub const MILLIMETER_PER_SECOND_CUBED: Unit = Unit::new(1, -3);
///The `Unit` for a quantity in millimeters per second squared (mm/s^2).
pub const MILLIMETER_PER_SECOND_SQUARED: Unit = Unit::new(1, -2);
///The `Unit` for a quantity in millimeters per second (mm/s).
pub const MILLIMETER_PER_SECOND: Unit = Unit::new(1, -1);
///The `Unit` for a quantity in millimeters (mm).
pub const MILLIMETER: Unit = Unit::new(1, 0);
///The `Unit` for a quantity in millimeter seconds (mm·s).
pub const MILLIMETER_SECOND: Unit = Unit::new(1, 1);
///The `Unit` for a quantity in millimeter seconds squared (mm·s^2).
pub const MILLIMETER_SECOND_SQUARED: Unit = Unit::new(1, 2);
///The `Unit` for a quantity in millimeter seconds cubed (mm·s^3).
pub const MILLIMETER_SECOND_CUBED: Unit = Unit::new(1, 3);

///The `Unit` for a quantity in millimeters squared per second cubed (mm^2/s^3).
pub const MILLIMETER_SQUARED_PER_SECOND_CUBED: Unit = Unit::new(2, -3);
///The `Unit` for a quantity in millimeters squared per second squared (mm^2/s^2).
pub const MILLIMETER_SQUARED_PER_SECOND_SQUARED: Unit = Unit::new(2, -2);
///The `Unit` for a quantity in millimeters squared per second (mm^2/s).
pub const MILLIMETER_SQUARED_PER_SECOND: Unit = Unit::new(2, -1);
///The `Unit` for a quantity in millimeters squared (mm^2).
pub const MILLIMETER_SQUARED: Unit = Unit::new(2, 0);
///The `Unit` for a quantity in millimeters squared seconds (mm^2·s).
pub const MILLIMETER_SQUARED_SECOND: Unit = Unit::new(2, 1);
///The `Unit` for a quantity in millimeters squared seconds squared (mm^2·s^2).
pub const MILLIMETER_SQUARED_SECOND_SQUARED: Unit = Unit::new(2, 2);
///The `Unit` for a quantity in millimeters squared seconds cubed (mm^2·s^3).
pub const MILLIMETER_SQUARED_SECOND_CUBED: Unit = Unit::new(2, 3);

///The `Unit` for a quantity in millimeters cubed per second cubed (mm^3/s^3).
pub const MILLIMETER_CUBED_PER_SECOND_CUBED: Unit = Unit::new(3, -3);
///The `Unit` for a quantity in millimeters cubed per second squared (mm^3/s^2).
pub const MILLIMETER_CUBED_PER_SECOND_SQUARED: Unit = Unit::new(3, -2);
///The `Unit` for a quantity in millimeters cubed per second (mm^3/s).
pub const MILLIMETER_CUBED_PER_SECOND: Unit = Unit::new(3, -1);
///The `Unit` for a quantity in millimeters cubed (mm^3).
pub const MILLIMETER_CUBED: Unit = Unit::new(3, 0);
///The `Unit` for a quantity in millimeters cubed seconds (mm^3·s).
pub const MILLIMETER_CUBED_SECOND: Unit = Unit::new(3, 1);
///The `Unit` for a quantity in millimeters cubed seconds squared (mm^3·s^2).
pub const MILLIMETER_CUBED_SECOND_SQUARED: Unit = Unit::new(3, 2);
///The `Unit` for a quantity in millimeters cubed seconds cubed (mm^3·s^3).
pub const MILLIMETER_CUBED_SECOND_CUBED: Unit = Unit::new(3, 3);
