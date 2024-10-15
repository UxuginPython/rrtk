//!Contains `Reference`, a special enum vith variants for different kinds of references, and
//!related types. Everything here is reexported at the crate level.
use crate::*;
#[cfg(feature = "alloc")]
use core::cell::{Ref, RefMut};
#[cfg(feature = "std")]
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
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
    ///A `MutexGuard`.
    #[cfg(feature = "std")]
    MutexGuard(MutexGuard<'a, T>),
}
impl<T: ?Sized> Deref for Borrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRef(ref_cell_ref) => ref_cell_ref,
            #[cfg(feature = "std")]
            Self::RwLockReadGuard(rw_lock_read_guard) => rw_lock_read_guard,
            #[cfg(feature = "std")]
            Self::MutexGuard(mutex_guard) => mutex_guard,
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
    ///A `MutexGuard`.
    #[cfg(feature = "std")]
    MutexGuard(MutexGuard<'a, T>),
}
impl<T: ?Sized> Deref for BorrowMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(ref_cell_ref_mut) => ref_cell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
            #[cfg(feature = "std")]
            Self::MutexGuard(mutex_guard) => mutex_guard,
        }
    }
}
impl<T: ?Sized> DerefMut for BorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Ptr(ptr, _) => unsafe { &mut **ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(ref_cell_ref_mut) => ref_cell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
            #[cfg(feature = "std")]
            Self::MutexGuard(mutex_guard) => mutex_guard,
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
    ///A raw pointer to a `Mutex<T>`.
    #[cfg(feature = "std")]
    PtrMutex(*const Mutex<T>),
    ///An `Arc<RwLock<T>>`.
    #[cfg(feature = "std")]
    ArcRwLock(Arc<RwLock<T>>),
    ///An `Arc<Mutex<T>>`.
    #[cfg(feature = "std")]
    ArcMutex(Arc<Mutex<T>>),
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
    pub const fn from_rc_ref_cell(rc_ref_cell: Rc<RefCell<T>>) -> Self {
        Self::RcRefCell(rc_ref_cell)
    }
    ///Create a `Reference` from a `*const RwLock<T>`.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_rw_lock(ptr_rw_lock: *const RwLock<T>) -> Self {
        Self::PtrRwLock(ptr_rw_lock)
    }
    ///Create a `Reference` from a `*const Mutex<T>`.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_mutex(ptr_mutex: *const Mutex<T>) -> Self {
        Self::PtrMutex(ptr_mutex)
    }
    ///Create a new `Reference` from an `Arc<RwLock<T>>`.
    #[cfg(feature = "std")]
    pub const fn from_arc_rw_lock(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
    ///Create a `Reference` from an `Arc<Mutex<T>>`.
    #[cfg(feature = "std")]
    pub const fn from_arc_mutex(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
    ///Immutably borrow the reference like a `RefCell`.
    pub fn borrow(&self) -> Borrow<'_, T> {
        match self {
            Self::Ptr(ptr) => Borrow::Ptr(*ptr, PhantomData),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_ref_cell) => Borrow::RefCellRef(rc_ref_cell.borrow()),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                Borrow::RwLockReadGuard(
                    (**ptr_rw_lock)
                        .read()
                        .expect("RRTK Reference borrow failed to get RwLock read lock"),
                )
            },
            #[cfg(feature = "std")]
            Self::PtrMutex(ptr_mutex) => unsafe {
                Borrow::MutexGuard(
                    (**ptr_mutex)
                        .lock()
                        .expect("RRTK Reference borrow failed to get Mutex lock"),
                )
            },
            #[cfg(feature = "std")]
            Self::ArcRwLock(arc_rw_lock) => Borrow::RwLockReadGuard(
                arc_rw_lock
                    .read()
                    .expect("RRTK Reference borrow failed to get RwLock read lock"),
            ),
            #[cfg(feature = "std")]
            Self::ArcMutex(arc_mutex) => Borrow::MutexGuard(
                arc_mutex
                    .lock()
                    .expect("RRTK Reference borrow failed to get Mutex lock"),
            ),
        }
    }
    ///Mutably borrow the reference like a `RefCell`.
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        match self {
            Self::Ptr(ptr) => BorrowMut::Ptr(*ptr, PhantomData),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_ref_cell) => BorrowMut::RefCellRefMut(rc_ref_cell.borrow_mut()),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                BorrowMut::RwLockWriteGuard(
                    (**ptr_rw_lock)
                        .write()
                        .expect("RRTK Reference mutable borrow failed to get RwLock write lock"),
                )
            },
            #[cfg(feature = "std")]
            Self::PtrMutex(ptr_mutex) => unsafe {
                BorrowMut::MutexGuard(
                    (**ptr_mutex)
                        .lock()
                        .expect("RRTK Reference mutable borrow failed to get Mutex lock"),
                )
            },
            #[cfg(feature = "std")]
            Self::ArcRwLock(arc_rw_lock) => BorrowMut::RwLockWriteGuard(
                arc_rw_lock
                    .write()
                    .expect("RRTK Reference mutable borrow failed to get RwLock write lock"),
            ),
            #[cfg(feature = "std")]
            Self::ArcMutex(arc_mutex) => BorrowMut::MutexGuard(
                arc_mutex
                    .lock()
                    .expect("RRTK Reference mutable borrow failed to get Mutex lock"),
            ),
        }
    }
}
impl<T: ?Sized> Clone for Reference<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Ptr(ptr) => Self::Ptr(*ptr),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_ref_cell) => Self::RcRefCell(Rc::clone(&rc_ref_cell)),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => Self::PtrRwLock(*ptr_rw_lock),
            #[cfg(feature = "std")]
            Self::PtrMutex(ptr_mutex) => Self::PtrMutex(*ptr_mutex),
            #[cfg(feature = "std")]
            Self::ArcRwLock(arc_rw_lock) => Self::ArcRwLock(Arc::clone(&arc_rw_lock)),
            #[cfg(feature = "std")]
            Self::ArcMutex(arc_mutex) => Self::ArcMutex(Arc::clone(&arc_mutex)),
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
        #[allow(unreachable_patterns)]
        match $was {
            Reference::Ptr(ptr) => unsafe { Reference::from_ptr(ptr as *mut dyn $trait) },
            #[cfg(feature = "alloc")]
            Reference::RcRefCell(rc_ref_cell) => Reference::from_rc_ref_cell(
                rc_ref_cell as alloc::rc::Rc<core::cell::RefCell<dyn $trait>>,
            ),
            #[cfg(feature = "std")]
            Reference::PtrRwLock(ptr_rw_lock) => unsafe {
                Reference::from_ptr_rw_lock(ptr_rw_lock as *const std::sync::RwLock<dyn $trait>)
            },
            _ => unimplemented!(),
        }
    }};
}
