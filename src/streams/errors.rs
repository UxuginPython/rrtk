use core::fmt::Debug;
#[derive(Clone, Copy, Debug)]
pub enum StreamError<O: Copy + Debug> {
    ///Returned when a `SumStream` has no inputs.
    EmptyAddendVec,
    ///Returned when a `ProductStream` has no inputs.
    EmptyFactorVec,
    ///Returned when a `None` is elevated to an error by a `NoneToError`.
    FromNone,
    Other(O),
}
