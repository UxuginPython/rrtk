// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024-2025 UxuginPython
#![cfg(feature = "devices")]
use rrtk::devices::wrappers::*;
use rrtk::devices::*;
use rrtk::*;
#[test]
fn terminal() {
    let term1 = Terminal::<()>::new();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&term1.borrow()),
        Ok(None)
    );
    term1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0)
            )
        )))
    );
    let term2 = Terminal::<()>::new();
    connect(&term1, &term2);
    assert_eq!(
        term2.borrow().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0)
            )
        )))
    );
    term2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(5.0),
                InverseSecondSquared::new(6.0),
            ),
        ))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.5),
                InverseSecond::new(3.5),
                InverseSecondSquared::new(4.5)
            )
        )))
    );
    term1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
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
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(1.0)),
        ))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(1.0),
            InverseSecond::new(2.0),
            InverseSecondSquared::new(3.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(1.0))
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(-1.0),
            InverseSecond::new(-2.0),
            InverseSecondSquared::new(-3.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(-1.0))
    );

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(-1.0),
                InverseSecond::new(-2.0),
                InverseSecondSquared::new(-3.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(-1.0)),
        ))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(1.0),
            InverseSecond::new(2.0),
            InverseSecondSquared::new(3.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(1.0))
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(-1.0),
            InverseSecond::new(-2.0),
            InverseSecondSquared::new(-3.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(-1.0))
    );

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(-4.0),
                InverseSecond::new(-5.0),
                InverseSecondSquared::new(-6.0),
            ),
        ))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new((((1.0 + 4.0) / 2.0) + 1.0) / 2.0),
            InverseSecond::new(((2.0 + 5.0) / 2.0 + 2.0) / 2.0),
            InverseSecondSquared::new(((3.0 + 6.0) / 2.0 + 3.0) / 2.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(-(((1.0 + 4.0) / 2.0) + 4.0) / 2.0),
            InverseSecond::new(-((2.0 + 5.0) / 2.0 + 5.0) / 2.0),
            InverseSecondSquared::new(-((3.0 + 6.0) / 2.0 + 6.0) / 2.0)
        )
    );
}
#[test]
#[should_panic]
fn gear_train_1() {
    let _ = GearTrain::<'_, ()>::new([28.0]);
}
#[test]
fn gear_train_2() {
    let mut gear_train = GearTrain::<'_, ()>::new([12.0, 36.0]);
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    connect(gear_train.get_terminal_1(), &terminal1);
    connect(gear_train.get_terminal_2(), &terminal2);
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(None::<Datum<AngularState>>)
    );
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(6.0),
                InverseSecondSquared::new(9.0),
            ),
        ))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(3.0)),
        ))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(-1.0),
                InverseSecond::new(-2.0),
                InverseSecondSquared::new(-3.0)
            )
        )))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(-1.0))
        )))
    );
}
#[test]
fn gear_train_odd() {
    let mut gear_train = GearTrain::<'_, ()>::new([36.0, 12.0, 24.0]);
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    connect(gear_train.get_terminal_1(), &terminal1);
    connect(gear_train.get_terminal_2(), &terminal2);
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(None::<Datum<AngularState>>)
    );
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(6.0),
            ),
        ))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(2.0)),
        ))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(6.0),
                InverseSecondSquared::new(9.0)
            )
        )))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(3.0))
        )))
    );
}
#[test]
fn gear_train_even() {
    let mut gear_train = GearTrain::<'_, ()>::new([36.0, 12.0, 12.0, 24.0]);
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    connect(gear_train.get_terminal_1(), &terminal1);
    connect(gear_train.get_terminal_2(), &terminal2);
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(None::<Datum<AngularState>>)
    );
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(6.0),
            ),
        ))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(2.0)),
        ))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(-3.0),
                InverseSecond::new(-6.0),
                InverseSecondSquared::new(-9.0)
            )
        )))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(-3.0))
        )))
    );
}
#[test]
fn gear_train_multiple_inputs() {
    let mut gear_train = GearTrain::<'_, ()>::new([12.0, 24.0]);
    gear_train
        .get_terminal_1()
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(3),
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(6.0),
            ),
        ))
        .unwrap();
    gear_train
        .get_terminal_1()
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(3),
            Command::Position(Dimensionless::new(2.0)),
        ))
        .unwrap();
    gear_train
        .get_terminal_2()
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(2),
            AngularState::new(
                Dimensionless::new(-2.0),
                InverseSecond::new(-4.0),
                InverseSecondSquared::new(-6.0),
            ),
        ))
        .unwrap();
    gear_train
        .get_terminal_2()
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(2),
            Command::Position(Dimensionless::new(-2.0)),
        ))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        gear_train.get_terminal_1().borrow().get(),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(3),
            AngularState::new(
                Dimensionless::new(2.4),
                InverseSecond::new(4.8),
                InverseSecondSquared::new(7.2)
            )
        )))
    );
    assert_eq!(
        gear_train.get_terminal_1().borrow().get(),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(3),
            Command::Position(Dimensionless::new(2.0))
        )))
    );
    assert_eq!(
        gear_train.get_terminal_2().borrow().get(),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(3),
            AngularState::new(
                Dimensionless::new(-1.2),
                InverseSecond::new(-2.4),
                InverseSecondSquared::new(-3.6)
            )
        )))
    );
    assert_eq!(
        gear_train.get_terminal_2().borrow().get(),
        Ok(Some(Datum::new(
            Time::from_nanoseconds(3),
            Command::Position(Dimensionless::new(-1.0))
        )))
    );
}
#[test]
fn axle() {
    let mut axle = Axle::<3, ()>::new();
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal3 = Terminal::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(5.0),
                InverseSecondSquared::new(6.0),
            ),
        ))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            Command::Position(Dimensionless::new(1.0)),
        ))
        .unwrap();
    connect(axle.get_terminal(0), &terminal1);
    connect(axle.get_terminal(1), &terminal2);
    connect(axle.get_terminal(2), &terminal3);
    axle.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(((1.0 + 4.0) / 2.0 + 1.0) / 2.0),
            InverseSecond::new(((2.0 + 5.0) / 2.0 + 2.0) / 2.0),
            InverseSecondSquared::new(((3.0 + 6.0) / 2.0 + 3.0) / 2.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(1.0))
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(((1.0 + 4.0) / 2.0 + 4.0) / 2.0),
            InverseSecond::new(((2.0 + 5.0) / 2.0 + 5.0) / 2.0),
            InverseSecondSquared::new(((3.0 + 6.0) / 2.0 + 6.0) / 2.0)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(1.0))
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal3.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(2.5),
            InverseSecond::new(3.5),
            InverseSecondSquared::new(4.5)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal3.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(Dimensionless::new(1.0))
    );
}
#[test]
fn differential() {
    let mut differential = Differential::<()>::new();
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal_sum = Terminal::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(2.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(3.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(4.0),
            ),
        ))
        .unwrap();
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
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_1),
            InverseSecond::new(TERM_1),
            InverseSecondSquared::new(TERM_1)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_2),
            InverseSecond::new(TERM_2),
            InverseSecondSquared::new(TERM_2)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_SUM),
            InverseSecond::new(TERM_SUM),
            InverseSecondSquared::new(TERM_SUM)
        )
    );
}
#[test]
fn differential_distrust_side_1() {
    let mut differential = Differential::<()>::with_distrust(DifferentialDistrust::Side1);
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal_sum = Terminal::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(2.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(3.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(4.0),
            ),
        ))
        .unwrap();
    connect(differential.get_side_1(), &terminal1);
    connect(differential.get_side_2(), &terminal2);
    connect(differential.get_sum(), &terminal_sum);
    differential.update().unwrap();
    const EST_1: f32 = 1.0;
    const EST_2: f32 = 3.0;
    const EST_SUM: f32 = 4.0;
    assert_eq!(EST_1 + EST_2, EST_SUM);
    const TERM_1: f32 = (EST_1 + 2.0) / 2.0;
    const TERM_2: f32 = (EST_2 + 3.0) / 2.0;
    const TERM_SUM: f32 = (EST_SUM + 4.0) / 2.0;
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_1),
            InverseSecond::new(TERM_1),
            InverseSecondSquared::new(TERM_1)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_2),
            InverseSecond::new(TERM_2),
            InverseSecondSquared::new(TERM_2)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_SUM),
            InverseSecond::new(TERM_SUM),
            InverseSecondSquared::new(TERM_SUM)
        )
    );
}
#[test]
fn differential_distrust_side_2() {
    let mut differential = Differential::<()>::with_distrust(DifferentialDistrust::Side2);
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal_sum = Terminal::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(2.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(3.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(4.0),
            ),
        ))
        .unwrap();
    connect(differential.get_side_1(), &terminal1);
    connect(differential.get_side_2(), &terminal2);
    connect(differential.get_sum(), &terminal_sum);
    differential.update().unwrap();
    const EST_1: f32 = 2.0;
    const EST_2: f32 = 2.0;
    const EST_SUM: f32 = 4.0;
    assert_eq!(EST_1 + EST_2, EST_SUM);
    const TERM_1: f32 = (EST_1 + 2.0) / 2.0;
    const TERM_2: f32 = (EST_2 + 3.0) / 2.0;
    const TERM_SUM: f32 = (EST_SUM + 4.0) / 2.0;
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_1),
            InverseSecond::new(TERM_1),
            InverseSecondSquared::new(TERM_1)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_2),
            InverseSecond::new(TERM_2),
            InverseSecondSquared::new(TERM_2)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_SUM),
            InverseSecond::new(TERM_SUM),
            InverseSecondSquared::new(TERM_SUM)
        )
    );
}
#[test]
fn differential_distrust_sum() {
    let mut differential = Differential::<()>::with_distrust(DifferentialDistrust::Sum);
    let terminal1 = Terminal::new();
    let terminal2 = Terminal::new();
    let terminal_sum = Terminal::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(2.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(2.0),
            ),
        ))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(3.0),
                InverseSecond::new(3.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(
            Time::ZERO,
            AngularState::new(
                Dimensionless::new(4.0),
                InverseSecond::new(4.0),
                InverseSecondSquared::new(4.0),
            ),
        ))
        .unwrap();
    connect(differential.get_side_1(), &terminal1);
    connect(differential.get_side_2(), &terminal2);
    connect(differential.get_sum(), &terminal_sum);
    differential.update().unwrap();
    const EST_1: f32 = 2.0;
    const EST_2: f32 = 3.0;
    const EST_SUM: f32 = 5.0;
    assert_eq!(EST_1 + EST_2, EST_SUM);
    const TERM_1: f32 = (EST_1 + 2.0) / 2.0;
    const TERM_2: f32 = (EST_2 + 3.0) / 2.0;
    const TERM_SUM: f32 = (EST_SUM + 4.0) / 2.0;
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_1),
            InverseSecond::new(TERM_1),
            InverseSecondSquared::new(TERM_1)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_2),
            InverseSecond::new(TERM_2),
            InverseSecondSquared::new(TERM_2)
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(TERM_SUM),
            InverseSecond::new(TERM_SUM),
            InverseSecondSquared::new(TERM_SUM)
        )
    );
}
//TODO: make this test more thorough with the different combinations of Some/None command and
//state.
#[test]
fn actuator_wrapper() {
    struct Actuator {
        last_request: Option<TerminalData>,
    }
    impl Actuator {
        fn new() -> Self {
            Self { last_request: None }
        }
    }
    impl Settable<TerminalData, ()> for Actuator {
        fn set(&mut self, x: TerminalData) -> NothingOrError<()> {
            self.last_request = Some(x);
            Ok(())
        }
    }
    impl Updatable<()> for Actuator {
        fn update(&mut self) -> NothingOrError<()> {
            assert_eq!(
                self.last_request.unwrap(),
                TerminalData {
                    time: Time::from_nanoseconds(2),
                    command: Some(Command::new(PositionDerivative::Position, 5.0)),
                    state: Some(AngularState::new(
                        Dimensionless::new(1.0),
                        InverseSecond::new(2.0),
                        InverseSecondSquared::new(3.0)
                    )),
                }
            );
            unsafe {
                ASSERTED = true;
            }
            Ok(())
        }
    }
    static mut ASSERTED: bool = false;
    let mut wrapper = ActuatorWrapper::new(Actuator::new());
    let terminal = Terminal::new();
    connect(wrapper.get_terminal(), &terminal);
    terminal
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(1),
            Command::new(PositionDerivative::Position, 5.0),
        ))
        .unwrap();
    terminal
        .borrow_mut()
        .set(Datum::new(
            Time::from_nanoseconds(2),
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(2.0),
                InverseSecondSquared::new(3.0),
            ),
        ))
        .unwrap();
    wrapper.update().unwrap();
    unsafe {
        assert!(ASSERTED);
    }
}
#[test]
fn getter_state_device_wrapper() {
    struct GetterState;
    impl Getter<AngularState, ()> for GetterState {
        fn get(&self) -> Output<AngularState, ()> {
            Ok(Some(Datum::new(
                Time::ZERO,
                AngularState::new(
                    Dimensionless::new(1.0),
                    InverseSecond::new(2.0),
                    InverseSecondSquared::new(3.0),
                ),
            )))
        }
    }
    impl Updatable<()> for GetterState {
        fn update(&mut self) -> NothingOrError<()> {
            Ok(())
        }
    }
    let mut wrapper = GetterStateDeviceWrapper::new(GetterState);
    let terminal = Terminal::new();
    connect(wrapper.get_terminal(), &terminal);
    wrapper.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<AngularState, ()>>::get(&terminal.borrow())
            .unwrap()
            .unwrap()
            .value,
        AngularState::new(
            Dimensionless::new(1.0),
            InverseSecond::new(2.0),
            InverseSecondSquared::new(3.0)
        )
    );
}
#[test]
#[cfg(feature = "alloc")]
fn pid_wrapper() {
    static mut ASSERTS: u8 = 0;
    const COMMAND: Command = Command::new(PositionDerivative::Position, 5.0);
    const STATE: AngularState = AngularState::new(
        Dimensionless::new(0.0),
        InverseSecond::new(0.0),
        InverseSecondSquared::new(0.0),
    );
    const K_VALUES: PositionDerivativeDependentPIDKValues =
        PositionDerivativeDependentPIDKValues::new(
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
        );
    use rrtk::*;
    struct Motor {
        time: Time,
    }
    impl Motor {
        fn new() -> Self {
            Self { time: Time::ZERO }
        }
    }
    impl Settable<f32, ()> for Motor {
        fn set(&mut self, value: f32) -> NothingOrError<()> {
            assert_eq!(
                value,
                match self.time.as_nanoseconds() {
                    0 => 5.0,
                    1_000_000_000 => 5.05,
                    2_000_000_000 => 5.1,
                    3_000_000_000 => 5.15,
                    4_000_000_000 => 5.20,
                    _ => unimplemented!(),
                }
            );
            unsafe {
                ASSERTS += 1;
            }
            Ok(())
        }
    }
    impl Updatable<()> for Motor {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    #[derive(Default)]
    struct Encoder {
        time: Time,
    }
    impl Getter<AngularState, ()> for Encoder {
        fn get(&self) -> Output<AngularState, ()> {
            Ok(Some(Datum::new(self.time, STATE)))
        }
    }
    impl Updatable<()> for Encoder {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time::from_nanoseconds(1_000_000_000);
            Ok(())
        }
    }
    let motor = Motor::new();
    let mut motor_wrapper =
        devices::wrappers::PIDWrapper::new(motor, Time::ZERO, STATE, COMMAND, K_VALUES);
    let encoder = Encoder::default();
    let mut encoder_wrapper = devices::wrappers::GetterStateDeviceWrapper::new(encoder);
    connect(motor_wrapper.get_terminal(), encoder_wrapper.get_terminal());
    for _ in 0..5 {
        motor_wrapper.update().unwrap();
        encoder_wrapper.update().unwrap();
    }
    #[allow(static_mut_refs)]
    unsafe {
        assert_eq!(ASSERTS, 5);
    }
}
