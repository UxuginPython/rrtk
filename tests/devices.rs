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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&term1.borrow()),
        Ok(None)
    );
    term1
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0))))
    );
    let term2 = Terminal::<()>::new();
    connect(&term1, &term2);
    assert_eq!(
        term2.borrow().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0))))
    );
    term2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 5.0, 6.0)))
        .unwrap();
    assert_eq!(
        term1.borrow().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(2.5, 3.5, 4.5))))
    );
    term1
        .borrow_mut()
        .set(Datum::new(
            Time(0),
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
        .set(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(1.0)))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(1.0, 2.0, 3.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(1.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(-1.0, -2.0, -3.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(-1.0)
    );

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(-1.0, -2.0, -3.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(-1.0)))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(1.0, 2.0, 3.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(1.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(-1.0, -2.0, -3.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(-1.0)
    );

    let mut invert = Invert::new();
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(-4.0, -5.0, -6.0)))
        .unwrap();
    connect(invert.get_terminal_1(), &terminal1);
    connect(invert.get_terminal_2(), &terminal2);
    invert.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(
            (((1.0 + 4.0) / 2.0) + 1.0) / 2.0,
            ((2.0 + 5.0) / 2.0 + 2.0) / 2.0,
            ((3.0 + 6.0) / 2.0 + 3.0) / 2.0
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(
            -(((1.0 + 4.0) / 2.0) + 4.0) / 2.0,
            -((2.0 + 5.0) / 2.0 + 5.0) / 2.0,
            -((3.0 + 6.0) / 2.0 + 6.0) / 2.0
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
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<State>>));
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(3.0, 6.0, 9.0)))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(3.0)))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(-1.0, -2.0, -3.0))))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), Command::Position(-1.0))))
    );
}
#[test]
fn gear_train_odd() {
    let mut gear_train = GearTrain::<'_, ()>::new([36.0, 12.0, 24.0]);
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    connect(gear_train.get_terminal_1(), &terminal1);
    connect(gear_train.get_terminal_2(), &terminal2);
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<State>>));
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0)))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(2.0)))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(3.0, 6.0, 9.0))))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), Command::Position(3.0))))
    );
}
#[test]
fn gear_train_even() {
    let mut gear_train = GearTrain::<'_, ()>::new([36.0, 12.0, 12.0, 24.0]);
    let terminal1 = Terminal::<()>::new();
    let terminal2 = Terminal::<()>::new();
    connect(gear_train.get_terminal_1(), &terminal1);
    connect(gear_train.get_terminal_2(), &terminal2);
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<State>>));
    assert_eq!(terminal2.borrow_mut().get(), Ok(None::<Datum<Command>>));
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(2.0, 4.0, 6.0)))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(2.0)))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), State::new_raw(-3.0, -6.0, -9.0))))
    );
    assert_eq!(
        terminal2.borrow_mut().get(),
        Ok(Some(Datum::new(Time(0), Command::Position(-3.0))))
    );
}
#[test]
fn gear_train_multiple_inputs() {
    let mut gear_train = GearTrain::<'_, ()>::new([12.0, 24.0]);
    gear_train
        .get_terminal_1()
        .borrow_mut()
        .set(Datum::new(Time(3), State::new_raw(2.0, 4.0, 6.0)))
        .unwrap();
    gear_train
        .get_terminal_1()
        .borrow_mut()
        .set(Datum::new(Time(3), Command::Position(2.0)))
        .unwrap();
    gear_train
        .get_terminal_2()
        .borrow_mut()
        .set(Datum::new(Time(2), State::new_raw(-2.0, -4.0, -6.0)))
        .unwrap();
    gear_train
        .get_terminal_2()
        .borrow_mut()
        .set(Datum::new(Time(2), Command::Position(-2.0)))
        .unwrap();
    gear_train.update().unwrap();
    assert_eq!(
        gear_train.get_terminal_1().borrow().get(),
        Ok(Some(Datum::new(Time(3), State::new_raw(2.4, 4.8, 7.2))))
    );
    assert_eq!(
        gear_train.get_terminal_1().borrow().get(),
        Ok(Some(Datum::new(Time(3), Command::Position(2.0))))
    );
    assert_eq!(
        gear_train.get_terminal_2().borrow().get(),
        Ok(Some(Datum::new(Time(3), State::new_raw(-1.2, -2.4, -3.6))))
    );
    assert_eq!(
        gear_train.get_terminal_2().borrow().get(),
        Ok(Some(Datum::new(Time(3), Command::Position(-1.0))))
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
        .set(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 5.0, 6.0)))
        .unwrap();
    terminal1
        .borrow_mut()
        .set(Datum::new(Time(0), Command::Position(1.0)))
        .unwrap();
    connect(axle.get_terminal(0), &terminal1);
    connect(axle.get_terminal(1), &terminal2);
    connect(axle.get_terminal(2), &terminal3);
    axle.update().unwrap();
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(
            ((1.0 + 4.0) / 2.0 + 1.0) / 2.0,
            ((2.0 + 5.0) / 2.0 + 2.0) / 2.0,
            ((3.0 + 6.0) / 2.0 + 3.0) / 2.0
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(1.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(
            ((1.0 + 4.0) / 2.0 + 4.0) / 2.0,
            ((2.0 + 5.0) / 2.0 + 5.0) / 2.0,
            ((3.0 + 6.0) / 2.0 + 6.0) / 2.0
        )
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(1.0)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal3.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(2.5, 3.5, 4.5)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<Command, ()>>::get(&terminal3.borrow())
            .unwrap()
            .unwrap()
            .value,
        Command::Position(1.0)
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
        .set(Datum::new(Time(0), State::new_raw(2.0, 2.0, 2.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(3.0, 3.0, 3.0)))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 4.0, 4.0)))
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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_1, TERM_1, TERM_1)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_2, TERM_2, TERM_2)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_SUM, TERM_SUM, TERM_SUM)
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
        .set(Datum::new(Time(0), State::new_raw(2.0, 2.0, 2.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(3.0, 3.0, 3.0)))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 4.0, 4.0)))
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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_1, TERM_1, TERM_1)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_2, TERM_2, TERM_2)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_SUM, TERM_SUM, TERM_SUM)
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
        .set(Datum::new(Time(0), State::new_raw(2.0, 2.0, 2.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(3.0, 3.0, 3.0)))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 4.0, 4.0)))
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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_1, TERM_1, TERM_1)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_2, TERM_2, TERM_2)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_SUM, TERM_SUM, TERM_SUM)
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
        .set(Datum::new(Time(0), State::new_raw(2.0, 2.0, 2.0)))
        .unwrap();
    terminal2
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(3.0, 3.0, 3.0)))
        .unwrap();
    terminal_sum
        .borrow_mut()
        .set(Datum::new(Time(0), State::new_raw(4.0, 4.0, 4.0)))
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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal1.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_1, TERM_1, TERM_1)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal2.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_2, TERM_2, TERM_2)
    );
    assert_eq!(
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal_sum.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(TERM_SUM, TERM_SUM, TERM_SUM)
    );
}
//TODO: make this test more thorough with the different combinations of Some/None command and
//state.
#[test]
fn actuator_wrapper() {
    struct Actuator {
        settable_data: SettableData<TerminalData, ()>,
    }
    impl Actuator {
        fn new() -> Self {
            Self {
                settable_data: SettableData::new(),
            }
        }
    }
    impl Settable<TerminalData, ()> for Actuator {
        fn get_settable_data_ref(&self) -> &SettableData<TerminalData, ()> {
            &self.settable_data
        }
        fn get_settable_data_mut(&mut self) -> &mut SettableData<TerminalData, ()> {
            &mut self.settable_data
        }
        fn impl_set(&mut self, _: TerminalData) -> NothingOrError<()> {
            Ok(())
        }
    }
    impl Updatable<()> for Actuator {
        fn update(&mut self) -> NothingOrError<()> {
            assert_eq!(
                self.get_last_request().unwrap(),
                TerminalData {
                    time: Time(2),
                    command: Some(Command::new(PositionDerivative::Position, 5.0)),
                    state: Some(State::new_raw(1.0, 2.0, 3.0)),
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
            Time(1),
            Command::new(PositionDerivative::Position, 5.0),
        ))
        .unwrap();
    terminal
        .borrow_mut()
        .set(Datum::new(Time(2), State::new_raw(1.0, 2.0, 3.0)))
        .unwrap();
    wrapper.update().unwrap();
    unsafe {
        assert!(ASSERTED);
    }
}
#[test]
fn getter_state_device_wrapper() {
    struct GetterState;
    impl Getter<State, ()> for GetterState {
        fn get(&self) -> Output<State, ()> {
            Ok(Some(Datum::new(Time(0), State::new_raw(1.0, 2.0, 3.0))))
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
        <rrtk::Terminal<'_, ()> as rrtk::Getter<State, ()>>::get(&terminal.borrow())
            .unwrap()
            .unwrap()
            .value,
        State::new_raw(1.0, 2.0, 3.0)
    );
}
#[test]
#[cfg(feature = "alloc")]
fn pid_wrapper() {
    static mut ASSERTS: u8 = 0;
    const COMMAND: Command = Command::new(PositionDerivative::Position, 5.0);
    const STATE: State = State::new_raw(0.0, 0.0, 0.0);
    const K_VALUES: PositionDerivativeDependentPIDKValues =
        PositionDerivativeDependentPIDKValues::new(
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
            PIDKValues::new(1.0, 0.01, 0.1),
        );
    use rrtk::*;
    struct Motor {
        settable_data: SettableData<f32, ()>,
        time: Time,
    }
    impl Motor {
        fn new() -> Self {
            Self {
                settable_data: SettableData::new(),
                time: Time(0),
            }
        }
    }
    impl Settable<f32, ()> for Motor {
        fn impl_set(&mut self, value: f32) -> NothingOrError<()> {
            assert_eq!(
                value,
                match self.time {
                    Time(1_000_000_000) => 5.0,
                    Time(2_000_000_000) => 5.05,
                    Time(3_000_000_000) => 5.1,
                    Time(4_000_000_000) => 5.15,
                    _ => unimplemented!(),
                }
            );
            unsafe {
                ASSERTS += 1;
            }
            Ok(())
        }
        fn get_settable_data_ref(&self) -> &SettableData<f32, ()> {
            &self.settable_data
        }
        fn get_settable_data_mut(&mut self) -> &mut SettableData<f32, ()> {
            &mut self.settable_data
        }
    }
    impl Updatable<()> for Motor {
        fn update(&mut self) -> NothingOrError<()> {
            self.update_following_data().unwrap();
            self.time += Time(1_000_000_000);
            Ok(())
        }
    }
    #[derive(Default)]
    struct Encoder {
        time: Time,
    }
    impl Getter<State, ()> for Encoder {
        fn get(&self) -> Output<State, ()> {
            Ok(Some(Datum::new(self.time, STATE)))
        }
    }
    impl Updatable<()> for Encoder {
        fn update(&mut self) -> NothingOrError<()> {
            self.time += Time(1_000_000_000);
            Ok(())
        }
    }
    let motor = Motor::new();
    let mut motor_wrapper =
        devices::wrappers::PIDWrapper::new(motor, Time(0), STATE, COMMAND, K_VALUES);
    let encoder = Encoder::default();
    let mut encoder_wrapper = devices::wrappers::GetterStateDeviceWrapper::new(encoder);
    connect(motor_wrapper.get_terminal(), encoder_wrapper.get_terminal());
    for _ in 0..5 {
        motor_wrapper.update().unwrap();
        encoder_wrapper.update().unwrap();
    }
    #[allow(static_mut_refs)]
    unsafe {
        assert_eq!(ASSERTS, 4);
    }
}
