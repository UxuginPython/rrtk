use rrtk::*;
use rrtk::streams::*;
/*#[test]
fn pid() {
    #[cfg(feature = "std")]
    use std::fmt::Debug;
    #[cfg(feature = "std")]
    use std::rc::Rc;
    #[cfg(feature = "std")]
    use std::cell::RefCell;
    #[cfg(not(feature = "std"))]
    use core::fmt::Debug;
    #[cfg(not(feature = "std"))]
    use core::cell::RefCell;
    #[cfg(not(feature = "std"))]
    extern crate alloc;
    #[cfg(not(feature = "std"))]
    use alloc::rc::Rc;
    struct DummyInput {
        update_count: f32,
    }
    impl DummyInput {
        pub fn new() -> Self {
            Self {
                update_count: 0.0,
            }
        }
    }
    impl<E: Copy + Debug> Stream<f32, E> for DummyInput {
        fn get(&self) -> StreamOutput<f32, E> {
            Ok(Some(Datum::new(2.0 * self.update_count + 1.0, self.update_count)))
        }
        fn update(&mut self) {
            self.update_count += 1.0;
        }
    }
    let input = Rc::new(RefCell::new(Box::new(DummyInput::new()) as Box<dyn Stream<f32, u8>>));
    let pid = StreamPIDController::new(Rc::clone(&input), 5.0, 1.0, 0.01, 0.1);
    input.borrow_mut().update();
    assert_eq!(pid.get().unwrap().unwrap().value, 5.0);
    input.borrow_mut().update();
    assert_eq!(pid.get().unwrap().unwrap().value, 4.04);
}*/
/*#[test]
fn p() {
    #[cfg(feature = "std")]
    use std::fmt::Debug;
    #[cfg(feature = "std")]
    use std::rc::Rc;
    #[cfg(feature = "std")]
    use std::cell::RefCell;
    #[cfg(not(feature = "std"))]
    use core::fmt::Debug;
    #[cfg(not(feature = "std"))]
    use core::cell::RefCell;
    #[cfg(not(feature = "std"))]
    extern crate alloc;
    #[cfg(not(feature = "std"))]
    use alloc::rc::Rc;
    struct DummyInput {
        update_count: f32,
    }
    impl DummyInput {
        pub fn new() -> Self {
            Self {
                update_count: 0.0,
            }
        }
    }
    impl<E: Copy + Debug> Stream<f32, E> for DummyInput {
        fn get(&self) -> StreamOutput<f32, E> {
            Ok(Some(Datum::new(2.0 * self.update_count + 1.0, self.update_count)))
        }
        fn update(&mut self) {
            self.update_count += 1.0;
        }
    }
    let input = Rc::new(RefCell::new(Box::new(DummyInput::new()) as Box<dyn Stream<f32, u8>>));
    let pid = StreamPIDController::new(Rc::clone(&input), 5.0, 1.0, 0.0, 0.0);
    input.borrow_mut().update();
    println!("{:?}", input.borrow().get());
    assert_eq!(pid.get().unwrap().unwrap().value, 5.0);
    input.borrow_mut().update();
    assert_eq!(pid.get().unwrap().unwrap().value, 4.04);
}*/
