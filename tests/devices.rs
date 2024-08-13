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
use rrtk::devices::*;
#[test]
fn terminal() {
    let term1 = Terminal::<()>::new();
    assert_eq!(term1.borrow().get(), Ok(None));
    term1
        .borrow_mut()
        .set(Datum::new(0, State::new(1.0, 2.0, 3.0)))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(0, State::new(1.0, 2.0, 3.0))))
    );
    let term2 = Terminal::<()>::new();
    connect(&term1, &term2);
    assert_eq!(
        term2.borrow().get(),
        Ok(Some(Datum::new(0, State::new(1.0, 2.0, 3.0))))
    );
    term2
        .borrow_mut()
        .set(Datum::new(0, State::new(4.0, 5.0, 6.0)))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(0, State::new(2.5, 3.5, 4.5))))
    );
    term1
        .borrow_mut()
        .set(Datum::new(
            0,
            Command::new(PositionDerivative::Position, 1.0),
        ))
        .unwrap(); //The stuff from `Settable` should take care of everything.
    term1.borrow_mut().update().unwrap(); //This should do nothing.
}
#[test]
fn invert() {
    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal1.borrow_mut().set(Datum::new(0, State::new(1.0, 2.0, 3.0))).unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(terminal1.borrow().get().unwrap().unwrap().value, State::new(1.0, 2.0, 3.0));
    assert_eq!(terminal2.borrow().get().unwrap().unwrap().value, State::new(-1.0, -2.0, -3.0));

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal2.borrow_mut().set(Datum::new(0, State::new(-1.0, -2.0, -3.0))).unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(terminal1.borrow().get().unwrap().unwrap().value, State::new(1.0, 2.0, 3.0));
    assert_eq!(terminal2.borrow().get().unwrap().unwrap().value, State::new(-1.0, -2.0, -3.0));

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal1.borrow_mut().set(Datum::new(0, State::new(1.0, 2.0, 3.0))).unwrap();
    terminal2.borrow_mut().set(Datum::new(0, State::new(-4.0, -5.0, -6.0))).unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(terminal1.borrow().get().unwrap().unwrap().value, State::new((((1.0 + 4.0) / 2.0) + 1.0) / 2.0, ((2.0 + 5.0) / 2.0 + 2.0) / 2.0, ((3.0 + 6.0) / 2.0 + 3.0) / 2.0));
    assert_eq!(terminal2.borrow().get().unwrap().unwrap().value, State::new(-(((1.0 + 4.0) / 2.0) + 4.0) / 2.0, -((2.0 + 5.0) / 2.0 + 5.0) / 2.0, -((3.0 + 6.0) / 2.0 + 6.0) / 2.0));
}
#[test]
fn axle() {
    let mut axle = Axle::<3, ()>::new();
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal3 = Terminal::new();
    terminal1.borrow_mut().set(Datum::new(0, State::new(1.0, 2.0, 3.0))).unwrap();
    terminal2.borrow_mut().set(Datum::new(0, State::new(4.0, 5.0, 6.0))).unwrap();
    connect(axle.get_terminal(0), &terminal1);
    connect(axle.get_terminal(1), &terminal2);
    connect(axle.get_terminal(2), &terminal3);
    axle.update().unwrap();
    assert_eq!(terminal1.borrow().get().unwrap().unwrap().value, State::new(((1.0 + 4.0) / 2.0 + 1.0) / 2.0, ((2.0 + 5.0) / 2.0 + 2.0) / 2.0, ((3.0 + 6.0) / 2.0 + 3.0) / 2.0));
    assert_eq!(terminal2.borrow().get().unwrap().unwrap().value, State::new(((1.0 + 4.0) / 2.0 + 4.0) / 2.0, ((2.0 + 5.0) / 2.0 + 5.0) / 2.0, ((3.0 + 6.0) / 2.0 + 6.0) / 2.0));
    assert_eq!(terminal3.borrow().get().unwrap().unwrap().value, State::new(2.5, 3.5, 4.5));
}
#[test]
fn differential() {
    let mut differential = Differential::<()>::new();
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal_sum = Terminal::new();
    terminal1.borrow_mut().set(Datum::new(0, State::new(2.0, 2.0, 2.0))).unwrap();
    terminal2.borrow_mut().set(Datum::new(0, State::new(3.0, 3.0, 3.0))).unwrap();
    terminal_sum.borrow_mut().set(Datum::new(0, State::new(4.0, 4.0, 4.0))).unwrap();
    connect(differential.get_side_1(), &terminal1);
    connect(differential.get_side_2(), &terminal2);
    connect(differential.get_sum(), &terminal_sum);
    differential.update().unwrap();
    const EST_1: f32 = 1.6666666666;
    const EST_2: f32 = 2.6666666666;
    const EST_SUM: f32 = 4.333333333333;
    assert_eq!(EST_1 + EST_2, EST_SUM);
    const TERM_1: f32 = (EST_1 + 2.0) / 2.0;
    const TERM_2: f32 = (EST_2 + 3.0) / 2.0;
    const TERM_SUM: f32 = (EST_SUM + 4.0) / 2.0;
    assert_eq!(terminal1.borrow().get().unwrap().unwrap().value, State::new(TERM_1, TERM_1, TERM_1));
    assert_eq!(terminal2.borrow().get().unwrap().unwrap().value, State::new(TERM_2, TERM_2, TERM_2));
    assert_eq!(terminal_sum.borrow().get().unwrap().unwrap().value, State::new(TERM_SUM, TERM_SUM, TERM_SUM));
}
