use rrtk::new_devices::*;
use rrtk::*;
fn display(system: &System<3>, differential: &Differential) {
    println!(
        "{:?} + {:?} = {:?}",
        system
            .get_terminal_state(differential.side_a)
            .value
            .velocity
            .into_inner(),
        system
            .get_terminal_state(differential.side_b)
            .value
            .velocity
            .into_inner(),
        system
            .get_terminal_state(differential.sum_side)
            .value
            .velocity
            .into_inner()
    );
}
fn main() {
    let mut system = System::<3>::new();
    let mut differential = Differential::new(&mut system).unwrap();
    display(&system, &differential);
    system.set_terminal_state(
        differential.side_a,
        Datum::new(
            Time::from_seconds(Second::new(1.0)),
            AngularState::new(
                Dimensionless::new(1.0),
                InverseSecond::new(1.0),
                InverseSecondSquared::new(0.0),
            ),
        ),
    );
    display(&system, &differential);
    differential.update_device::<3>(&mut system).unwrap();
    display(&system, &differential);
}
