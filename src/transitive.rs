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
impl<'a, T, const N: usize> IntoIterator for &'a VecArray<T, N> {
    type Item = &'a T;
    type IntoIter = VecArrayIterator<'a, T, N>;
    #[inline]
    fn into_iter(self) -> VecArrayIterator<'a, T, N> {
        VecArrayIterator {
            next_index: 0,
            vec_array: self,
        }
    }
}
struct VecArrayIterator<'a, T, const N: usize> {
    next_index: usize,
    vec_array: &'a VecArray<T, N>,
}
impl<T, const N: usize> VecArrayIterator<'_, T, N> {
    ///Operates exactly like [`Iterator::count`] except that it only requires `&self` rather than
    ///`self` and it takes O(1) time rather than O(n). The default implementations of
    ///[`Iterator::count`] and [`Iterator::size_hint`] have been overridden using this method.
    ///Calling this method is preferred to calling either of those where possible.
    #[inline]
    pub const fn count_ref(&self) -> usize {
        self.vec_array.len() - self.next_index
    }
}
impl<'a, T, const N: usize> Iterator for VecArrayIterator<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.next_index >= self.vec_array.len() {
            return None;
        }
        self.next_index += 1;
        Some(self.vec_array.get(self.next_index - 1))
    }
    ///The default implementation is overridden to use [`count_ref`](Self::count_ref). Calling that
    ///method directly is preferred where possible.
    #[inline]
    fn count(self) -> usize {
        self.count_ref()
    }
    ///This implementation returns an exact value using [`count_ref`](Self::count_ref). Calling
    ///that method directly is preferred where possible.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.count_ref();
        (size, Some(size))
    }
    ///The default implementation is overridden to take O(1) time rather than O(n).
    #[inline]
    fn last(self) -> Option<&'a T> {
        if self.count_ref() >= 1 {
            Some(self.vec_array.get(self.vec_array.len() - 1))
        } else {
            None
        }
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
