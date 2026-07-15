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
