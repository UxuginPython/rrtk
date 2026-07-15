use rrtk::*;
const RETURN_1: Output<u8, ()> = Ok(Some(Datum::new(Time::from_nanoseconds(0), 5)));
struct TestGetter1;
impl Updatable<()> for TestGetter1 {
    fn update(&mut self) -> NothingOrError<()> {
        Ok(())
    }
}
impl Getter<u8, ()> for TestGetter1 {
    fn get(&self) -> Output<u8, ()> {
        RETURN_1
    }
}
#[test]
fn pointer_dereferencer() {
    let mut original = TestGetter1;
    assert_eq!(original.get(), RETURN_1);
    let ptr = &raw mut original;
    assert_eq!(unsafe { ptr.as_ref() }.unwrap().get(), RETURN_1);
    let deref = unsafe { PointerDereferencer::new(ptr) };
    assert_eq!(deref.get(), RETURN_1);
}
#[test]
fn as_dyn_getter() {
    let mut original = TestGetter1;
    assert_eq!(original.get(), RETURN_1);
    let ptr = &raw mut original;
    assert_eq!(unsafe { ptr.as_ref() }.unwrap().get(), RETURN_1);
    let deref = unsafe { PointerDereferencer::new(ptr) };
    assert_eq!(deref.get(), RETURN_1);
    let deref_dyn = deref.as_dyn_getter();
    assert_eq!(deref_dyn.get(), RETURN_1);
}
struct TestUpdatable2(*mut u8);
impl Updatable<()> for TestUpdatable2 {
    fn update(&mut self) -> NothingOrError<()> {
        *unsafe { self.0.as_mut() }.unwrap() += 1;
        Ok(())
    }
}
#[test]
fn as_dyn_updatable() {
    let mut value = 23u8;
    let mut original = TestUpdatable2(&raw mut value);
    original.update().unwrap();
    assert_eq!(value, 24);
    let ptr = &raw mut original;
    let mut deref = unsafe { PointerDereferencer::new(ptr) };
    deref.update().unwrap();
    assert_eq!(value, 25);
    let mut deref_dyn = deref.as_dyn_updatable();
    deref_dyn.update().unwrap();
    assert_eq!(value, 26);
}
struct TestSettable3(*mut u8);
impl Updatable<()> for TestSettable3 {
    fn update(&mut self) -> NothingOrError<()> {
        Ok(())
    }
}
impl Settable<u8, ()> for TestSettable3 {
    fn set(&mut self, value: u8) -> NothingOrError<()> {
        unsafe {
            *self.0 = value;
        }
        Ok(())
    }
}
#[test]
fn as_dyn_settable() {
    let mut value = 4u8;
    let mut original = TestSettable3(&raw mut value);
    original.set(6).unwrap();
    assert_eq!(value, 6);
    let ptr = &raw mut original;
    let mut deref = unsafe { PointerDereferencer::new(ptr) };
    deref.set(7).unwrap();
    assert_eq!(value, 7);
    let mut deref_dyn = deref.as_dyn_settable();
    deref_dyn.set(90).unwrap();
    assert_eq!(value, 90);
}
