//!Contains `Reference`, a special enum vith variants for different kinds of references, and
//!related types. Everything here is reexported at the crate level.
use crate::*;
#[cfg(feature = "alloc")]
use core::cell::{Ref, RefMut};
#[cfg(feature = "std")]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
///An immutable borrow of an RRTK `Reference`, similar to `Ref` for a `RefCell`.
///This is marked as non-exhaustive because some variants are only available with some features.
///This means that if you write a `match` without all the features enabled, it won't cover all the
///variants if another crate in the tree enables more features. This is a problem because features
///are additive, so it is marked as non-exhaustive to remedy this.
#[non_exhaustive]
pub enum Borrow<'a, T: ?Sized> {
    ///A raw immutable pointer.
    Ptr(*const T, PhantomData<&'a ()>),
    ///An immutable borrow of an `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    RefCellRef(Ref<'a, T>),
    ///An `RwLockReadGuard`.
    #[cfg(feature = "std")]
    RwLockReadGuard(RwLockReadGuard<'a, T>),
}
impl<T: ?Sized> Deref for Borrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRef(refcell_ref) => refcell_ref,
            #[cfg(feature = "std")]
            Self::RwLockReadGuard(rw_lock_read_guard) => rw_lock_read_guard,
        }
    }
}
///A mutable borrow of an RRTK `Reference`, similar to `RefMut` for a `RefCell`.
///This is marked as non-exhaustive because some variants are only available with some features.
///This means that if you write a `match` without all the features enabled, it won't cover all the
///variants if another crate in the tree enables more features. This is a problem because features
///are additive, so it is marked as non-exhaustive to remedy this.
#[non_exhaustive]
pub enum BorrowMut<'a, T: ?Sized> {
    ///A raw mutable pointer.
    Ptr(*mut T, PhantomData<&'a ()>),
    ///A mutable borrow of an `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    RefCellRefMut(RefMut<'a, T>),
    ///An `RwLockWriteGuard`.
    #[cfg(feature = "std")]
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
}
impl<T: ?Sized> Deref for BorrowMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
        }
    }
}
impl<T: ?Sized> DerefMut for BorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &mut **ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
        }
    }
}
///A special enum with variants for different kinds of references depending on your platform and
///code structure. (Some variants are alloc- or std-only.)
///This is marked as non-exhaustive because some variants are only available with some features.
///This means that if you write a `match` without all the features enabled, it won't cover all the
///variants if another crate in the tree enables more features. This is a problem because features
///are additive, so it is marked as non-exhaustive to remedy this.
#[non_exhaustive]
pub enum Reference<T: ?Sized> {
    ///A raw mutable pointer.
    Ptr(*mut T),
    ///An `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    RcRefCell(Rc<RefCell<T>>),
    ///A raw immutable pointer to an `RwLock<T>`.
    #[cfg(feature = "std")]
    PtrRwLock(*const RwLock<T>),
}
impl<T: ?Sized> Reference<T> {
    ///Create a `Reference` from a raw mutable pointer. Good if you're not multithreading and you
    ///want to avoid dynamic allocation. Making the object static is strongly recommended if you
    ///use this.
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self::Ptr(ptr)
    }
    ///Create a `Reference` from an `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    pub const fn from_rc_refcell(rc_refcell: Rc<RefCell<T>>) -> Self {
        Self::RcRefCell(rc_refcell)
    }
    ///Create a `Reference` from a `*const RwLock<T>`.
    #[cfg(feature = "std")]
    pub const unsafe fn from_rwlock_ptr(ptr_rwlock: *const RwLock<T>) -> Self {
        Self::PtrRwLock(ptr_rwlock)
    }
    ///Immutably borrow the reference like a `RefCell`.
    pub fn borrow(&self) -> Borrow<'_, T> {
        match self {
            Self::Ptr(ptr) => Borrow::Ptr(*ptr, PhantomData),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_refcell) => Borrow::RefCellRef(rc_refcell.borrow()),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                Borrow::RwLockReadGuard(
                    (**ptr_rw_lock)
                        .read()
                        .expect("RRTK Reference borrow failed to get RwLock read lock"),
                )
            },
        }
    }
    ///Mutably borrow the reference like a `RefCell`.
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        match self {
            Self::Ptr(ptr) => BorrowMut::Ptr(*ptr, PhantomData),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_refcell) => BorrowMut::RefCellRefMut(rc_refcell.borrow_mut()),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                BorrowMut::RwLockWriteGuard(
                    (**ptr_rw_lock)
                        .write()
                        .expect("RRTK Reference mutable borrow failed to get RwLock write lock"),
                )
            },
        }
    }
}
impl<T: ?Sized> Clone for Reference<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Ptr(ptr) => Self::Ptr(*ptr),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_refcell) => Self::RcRefCell(Rc::clone(&rc_refcell)),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rwlock) => Self::PtrRwLock(*ptr_rwlock),
        }
    }
}
///If you have a `Reference<Foo>` where `Foo` implements the `Bar` trait, you may end up wanting a
///`Reference<dyn Bar>`. To convert it, you would do this:
///```
///# use rrtk::*;
///# struct Foo;
///# impl Foo {
///#     fn foo_func(&self) {}
///# }
///# trait Bar {
///#     fn bar_func(&self) {}
///# }
///# impl Bar for Foo {}
///static mut FOO: Foo = Foo;
///unsafe {
///    let ref_foo = Reference::from_ptr(core::ptr::addr_of_mut!(FOO));
///    ref_foo.borrow().foo_func();
///    ref_foo.borrow().bar_func();
///    let ref_dyn_bar = to_dyn!(Bar, ref_foo);
///    //ref_dyn_bar.borrow().foo_func(); //It won't compile with this.
///    ref_dyn_bar.borrow().bar_func();
///}
///```
#[macro_export]
macro_rules! to_dyn {
    ($trait:path, $was:expr) => {{
        #[cfg(feature = "alloc")]
        extern crate alloc;
        match $was {
            Reference::Ptr(ptr) => unsafe { Reference::from_ptr(ptr as *mut dyn $trait) },
            #[cfg(feature = "alloc")]
            Reference::RcRefCell(rc_refcell) => Reference::from_rc_refcell(
                rc_refcell as alloc::rc::Rc<core::cell::RefCell<dyn $trait>>,
            ),
            #[cfg(feature = "std")]
            Reference::PtrRwLock(ptr_rw_lock) => unsafe {
                Reference::from_rwlock_ptr(ptr_rw_lock as *const std::sync::RwLock<dyn $trait>)
            },
            _ => unimplemented!(),
        }
    }};
}
