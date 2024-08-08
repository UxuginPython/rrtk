// SPDX-License-Identifier: LGPL-3.0-only
/*
Copyright 2024 UxuginPython on GitHub

     This file is part of Rust Robotics ToolKit.

    Rust Robotics ToolKit is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, version 3.

    Rust Robotics ToolKit is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License along with Rust Robotics ToolKit. If not, see <https://www.gnu.org/licenses/>.
*/
#![cfg(feature = "devices")]
use rrtk::*;
#[test]
fn terminal() {
    let term1 = Terminal::<()>::new();
    assert_eq!(term1.borrow().get(), Ok(None));
    term1.borrow_mut().set(Datum::new(0, State::new(1.0, 2.0, 3.0))).unwrap();
    assert_eq!(term1.borrow().get(), Ok(Some(Datum::new(0, State::new(1.0, 2.0, 3.0)))));
    let term2 = Terminal::<()>::new();
    connect(&term1, &term2);
    assert_eq!(term2.borrow().get(), Ok(Some(Datum::new(0, State::new(1.0, 2.0, 3.0)))));
    term2.borrow_mut().set(Datum::new(0, State::new(4.0, 5.0, 6.0))).unwrap();
    assert_eq!(term1.borrow().get(), Ok(Some(Datum::new(0, State::new(2.5, 3.5, 4.5)))));
    term1.borrow_mut().set(Datum::new(0, Command::new(PositionDerivative::Position, 1.0))).unwrap(); //The stuff from `Settable` should take care of everything.
    term1.borrow_mut().update().unwrap(); //This should do nothing.
}
