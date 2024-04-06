use core::fmt::Debug;
#[derive(Clone, Copy, Debug)]
pub enum StreamError<O: Copy + Debug> {
    EmptyAddendVec,
    EmptyFactorVec,
    Other(O),
}
