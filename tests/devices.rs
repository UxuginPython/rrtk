#![cfg(feature = "devices")]
use rrtk::devices::provided::*;
use rrtk::devices::*;
use rrtk::*;
#[test]
fn clutch() {
    let mut system = System::<4>::new();
    let a_test = system.new_node().unwrap();
    let a_clutch = system.new_node().unwrap();
    system.connect(a_test, a_clutch);
    let b_test = system.new_node().unwrap();
    let b_clutch = system.new_node().unwrap();
    system.connect(b_test, b_clutch);
    const A: AngularState = AngularState::new(
        Dimensionless::new(1.0),
        InverseSecond::new(2.0),
        InverseSecondSquared::new(3.0),
    );
    const B: AngularState = AngularState::new(
        Dimensionless::new(4.0),
        InverseSecond::new(5.0),
        InverseSecondSquared::new(6.0),
    );
    system.set_state_local(a_test, Some(A));
    system.set_state_local(b_test, Some(B));

    let mut clutch = Clutch::new(a_clutch, b_clutch);
    clutch.device_update(&mut system);
    assert!(system.get_state_connected(a_test).is_none());
    assert!(system.get_state_connected(b_test).is_none());
    clutch.set_connected(true);
    clutch.device_update(&mut system);
    assert_eq!(system.get_state_connected(a_test), Some(B));
    assert_eq!(system.get_state_connected(b_test), Some(A));
    clutch.set_connected(false);
    clutch.device_update(&mut system);
    assert!(system.get_state_connected(a_test).is_none());
    assert!(system.get_state_connected(b_test).is_none());
}
#[test]
fn gear_train() {
    let mut system = System::<4>::new();
    let a_test = system.new_node().unwrap();
    let a_gear_train = system.new_node().unwrap();
    system.connect(a_test, a_gear_train);
    let b_test = system.new_node().unwrap();
    let b_gear_train = system.new_node().unwrap();
    system.connect(b_test, b_gear_train);
    const A: AngularState = AngularState::new(
        Dimensionless::new(1.0),
        InverseSecond::new(2.0),
        InverseSecondSquared::new(3.0),
    );
    const B: AngularState = AngularState::new(
        Dimensionless::new(4.0),
        InverseSecond::new(5.0),
        InverseSecondSquared::new(6.0),
    );
    system.set_state_local(a_test, Some(A));
    system.set_state_local(b_test, Some(B));

    let mut gear_train = GearTrain::new(a_gear_train, b_gear_train, Dimensionless::new(2.0));
    gear_train.device_update(&mut system);
    assert_eq!(
        system.get_state_connected(a_test),
        Some(AngularState::new(
            Dimensionless::new(2.0),
            InverseSecond::new(2.5),
            InverseSecondSquared::new(3.0),
        ))
    );
    assert_eq!(
        system.get_state_connected(b_test),
        Some(AngularState::new(
            Dimensionless::new(2.0),
            InverseSecond::new(4.0),
            InverseSecondSquared::new(6.0),
        ))
    );
}
#[test]
fn differential() {
    let mut system = System::<6>::new();
    let a_test = system.new_node().unwrap();
    let a_differential = system.new_node().unwrap();
    system.connect(a_test, a_differential);
    let b_test = system.new_node().unwrap();
    let b_differential = system.new_node().unwrap();
    system.connect(b_test, b_differential);
    let c_test = system.new_node().unwrap();
    let c_differential = system.new_node().unwrap();
    system.connect(c_test, c_differential);
    const A: AngularState = AngularState::new(
        Dimensionless::new(1.0),
        InverseSecond::new(2.0),
        InverseSecondSquared::new(3.0),
    );
    const B: AngularState = AngularState::new(
        Dimensionless::new(4.0),
        InverseSecond::new(5.0),
        InverseSecondSquared::new(6.0),
    );
    const C: AngularState = AngularState::new(
        Dimensionless::new(7.0),
        InverseSecond::new(8.0),
        InverseSecondSquared::new(9.0),
    );
    system.set_state_local(a_test, Some(A));
    system.set_state_local(b_test, Some(B));
    system.set_state_local(c_test, Some(C));

    let mut differential = Differential::new(a_differential, b_differential, c_differential);
    differential.device_update(&mut system);
    assert_eq!(system.get_state_connected(a_test), Some(C - B));
    assert_eq!(system.get_state_connected(b_test), Some(C - A));
    assert_eq!(system.get_state_connected(c_test), Some(A + B));
}
