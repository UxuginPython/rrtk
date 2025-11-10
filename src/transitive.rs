#![allow(unused)]
use core::mem::{MaybeUninit, replace};
struct VecArray<T, const N: usize> {
    inner: [MaybeUninit<T>; N],
    length: usize,
}
impl<T, const N: usize> VecArray<T, N> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            inner: [const { MaybeUninit::uninit() }; N],
            length: 0,
        }
    }
    #[inline]
    pub const fn clear(&mut self) {
        //A MaybeUninit is blind to whether or not it's initialized. We can just declare it to not
        //be so and let it be overwritten whenever; there's no need to actually clear it.
        self.length = 0;
    }
    #[inline]
    pub const fn len(&self) -> usize {
        self.length
    }
    #[inline]
    pub const fn get(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("rrtk::transitive::VecArray::get called out of bounds");
        }
        unsafe { self.inner[index].assume_init_ref() }
    }
    pub const fn push(&mut self, value: T) -> Result<(), T> {
        if self.length < N {
            self.inner[self.length].write(value);
            self.length += 1;
            Ok(())
        } else {
            Err(value)
        }
    }
    pub const fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            None
        } else {
            self.length -= 1;
            let to_return = replace(&mut self.inner[self.length], MaybeUninit::uninit());
            Some(unsafe { to_return.assume_init() })
        }
    }
    pub const fn swap_remove(&mut self, index: usize) -> T {
        if index >= self.length {
            panic!("rrtk::transitive::VecArray::swap_remove called out of bounds");
        }
        if self.length == 1 {
            return self.pop().expect("We know that we have length > 0, and the only way pop returns None is when length == 0.");
        }
        self.length -= 1;
        //We pop manually instead of using the method here to leave it as a MaybeUninit.
        let end_to_move = replace(&mut self.inner[self.length], MaybeUninit::uninit());
        let to_return = replace(&mut self.inner[index], end_to_move);
        unsafe { to_return.assume_init() }
    }
}
enum CacheAndGiveUp<T, const N: usize> {
    Cache(VecArray<T, N>),
    GiveUp,
}
impl<T, const N: usize> CacheAndGiveUp<T, N> {
    #[inline]
    pub const fn new() -> Self {
        Self::Cache(VecArray::new())
    }
    #[inline]
    pub const fn has_given_up(&self) -> bool {
        if let Self::GiveUp = self { true } else { false }
    }
}
