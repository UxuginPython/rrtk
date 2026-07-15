use rrtk::*;
const RETURN: Output<u8, ()> = Ok(Some(Datum::new(Time::from_nanoseconds(0), 5)));
struct MyGetter;
impl Updatable<()> for MyGetter {
    fn update(&mut self) -> NothingOrError<()> {
        Ok(())
    }
}
impl Getter<u8, ()> for MyGetter {
    fn get(&self) -> Output<u8, ()> {
        RETURN
    }
}
#[test]
fn pointer_dereferencer() {
    let mut original = MyGetter;
    assert_eq!(original.get(), RETURN);
    let ptr = &raw mut original;
    assert_eq!(unsafe { ptr.as_ref() }.unwrap().get(), RETURN);
    let deref = unsafe { PointerDereferencer::new(ptr) };
    assert_eq!(deref.get(), RETURN);
}
#[test]
fn as_dyn_getter() {
    let mut original = MyGetter;
    assert_eq!(original.get(), RETURN);
    let ptr = &raw mut original;
    assert_eq!(unsafe { ptr.as_ref() }.unwrap().get(), RETURN);
    let deref = unsafe { PointerDereferencer::new(ptr) };
    assert_eq!(deref.get(), RETURN);
    let deref_dyn = deref.as_dyn_getter();
    assert_eq!(deref_dyn.get(), RETURN);
}
