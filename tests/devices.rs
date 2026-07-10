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
