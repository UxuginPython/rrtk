//!`Reference` is a container privately holding an enum with variants containing different kinds of
//!references, the availability of some of which depends on crate features. `Reference` is borrowed like
//!a `RefCell`. This module contains it and its related types. `Reference` is also reexported at
//!the crate level.
use crate::*;
#[cfg(feature = "alloc")]
use core::cell::{Ref, RefMut};
#[cfg(feature = "std")]
use std::sync::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};
///An immutable borrow of an RRTK `Reference`, similar to `Ref` for a `RefCell`.
///
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
///
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
///code structure. (Some variants are alloc- or std-only.) It is usually contained in a
///`Reference`, which is a safe wrapper. You should generally use `Reference` over
///`ReferenceUnsafe` unless you specifically need to match against it, probably for some form of
///type conversion.
///
///This is marked as non-exhaustive because some variants are only available with some features.
///This means that if you write a `match` without all the features enabled, it won't cover all the
///variants if another crate in the tree enables more features. This is a problem because features
///are additive, so it is marked as non-exhaustive to remedy this.
#[non_exhaustive]
pub enum ReferenceUnsafe<T: ?Sized> {
    ///A raw mutable pointer. This is a useful variant if you are not multithreading and you want
    ///to avoid dynamic allocation. Making the object static is strongly recommended.
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
impl<T: ?Sized> ReferenceUnsafe<T> {
    ///Create a `ReferenceUnsafe` from a raw mutable pointer. This is useful if you are not
    ///multithreading and you want to avoid dynamic allocation. Making the object static is
    ///strongly recommended.
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self::Ptr(ptr)
    }
    ///Create a `ReferenceUnsafe` from an `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    pub const fn from_rc_ref_cell(rc_ref_cell: Rc<RefCell<T>>) -> Self {
        Self::RcRefCell(rc_ref_cell)
    }
    ///Create a `ReferenceUnsafe` from a `*const RwLock<T>`. Making the `RwLock` itself static is
    ///strongly recommended.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_rw_lock(ptr_rw_lock: *const RwLock<T>) -> Self {
        Self::PtrRwLock(ptr_rw_lock)
    }
    ///Create a `ReferenceUnsafe` from a `*const Mutex<T>`. Making the `Mutex` itself static is
    ///strongly recommended.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_mutex(ptr_mutex: *const Mutex<T>) -> Self {
        Self::PtrMutex(ptr_mutex)
    }
    ///Create a new `ReferenceUnsafe` from an `Arc<RwLock<T>>`.
    #[cfg(feature = "std")]
    pub const fn from_arc_rw_lock(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self::ArcRwLock(arc_rw_lock)
    }
    ///Create a `ReferenceUnsafe` from an `Arc<Mutex<T>>`.
    #[cfg(feature = "std")]
    pub const fn from_arc_mutex(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self::ArcMutex(arc_mutex)
    }
    ///Immutably borrow the `ReferenceUnsafe` like a `RefCell`. This is unsafe because of the
    ///potential for a dereference of the borrow to dereference a null or freed raw pointer.
    pub unsafe fn borrow(&self) -> Borrow<'_, T> {
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
    ///Mutably borrow the `ReferenceUnsafe` like a `RefCell`. Thus is unsafe because of the
    ///potential for a dereference of the borrow to dereference a null or freed raw pointer.
    pub unsafe fn borrow_mut(&self) -> BorrowMut<'_, T> {
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
impl<T: ?Sized> Clone for ReferenceUnsafe<T> {
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
///A container privately holding an enum with variants containing different kinds of references,
///the availability of some of which depends on crate features. `Reference` is borrowed like a `RefCell`.
///It is also reexported at the crate level.
#[repr(transparent)]
pub struct Reference<T: ?Sized>(ReferenceUnsafe<T>);
impl<T: ?Sized> Reference<T> {
    ///Create a `Reference` from a raw mutable pointer. This is useful if you are not
    ///multithreading and you want to avoid dynamic allocation. Making the object static is
    ///strongly recommended. The `static_reference!` macro is a convenient way of making the object
    ///static and getting a `Reference` of the raw pointer variant. Because the object is
    ///guaranteed to be static, it can be called without an unsafe block.
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self(ReferenceUnsafe::from_ptr(ptr))
    }
    ///Create a `Reference` from an `Rc<RefCell<T>>`. The `rc_ref_cell_reference` function is a
    ///convenient way of putting an object in an `Rc<RefCell>` and getting a `Reference` of this
    ///variant to it.
    #[cfg(feature = "alloc")]
    pub const fn from_rc_ref_cell(rc_ref_cell: Rc<RefCell<T>>) -> Self {
        Self(ReferenceUnsafe::from_rc_ref_cell(rc_ref_cell))
    }
    ///Create a `Reference` from a `*const RwLock<T>`. Making the `RwLock` itself static is
    ///strongly recommended. The `static_rw_lock_reference!` macro is a convenient way of putting
    ///an object in a static `RwLock` and getting a `Reference` of this variant to it.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_rw_lock(ptr_rw_lock: *const RwLock<T>) -> Self {
        Self(ReferenceUnsafe::from_ptr_rw_lock(ptr_rw_lock))
    }
    ///Create a `Reference` from a `*const Mutex<T>`. Making the `Mutex` itself static is strongly
    ///recommended. The `static_mutex_reference!` macro is a convenient way of putting an object in
    ///a static `Mutex` and getting a `Reference` of this variant to it.
    #[cfg(feature = "std")]
    pub const unsafe fn from_ptr_mutex(ptr_mutex: *const Mutex<T>) -> Self {
        Self(ReferenceUnsafe::from_ptr_mutex(ptr_mutex))
    }
    ///Create a `Reference` from an `Arc<RwLock<T>>`. The `arc_rw_lock_reference` function is a
    ///convenient way of putting an object in an `Arc<RwLock>` and getting a `Reference` of this
    ///variant to it.
    #[cfg(feature = "std")]
    pub const fn from_arc_rw_lock(arc_rw_lock: Arc<RwLock<T>>) -> Self {
        Self(ReferenceUnsafe::from_arc_rw_lock(arc_rw_lock))
    }
    ///Create a `Reference` from an `Arc<Mutex<T>>`. The `arc_mutex_reference` function is a
    ///convenient way of putting an object in an `Arc<Mutex>` and getting a `Reference` of this
    ///variant to it.
    #[cfg(feature = "std")]
    pub const fn from_arc_mutex(arc_mutex: Arc<Mutex<T>>) -> Self {
        Self(ReferenceUnsafe::from_arc_mutex(arc_mutex))
    }
    ///Get the inner `ReferenceUnsafe`.
    pub fn into_unsafe(self) -> ReferenceUnsafe<T> {
        self.0
    }
    ///Immutably borrow the `Reference` like a `RefCell`.
    pub fn borrow(&self) -> Borrow<'_, T> {
        unsafe { self.0.borrow() }
    }
    ///Mutably borrow the `Reference` like a `RefCell`.
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        unsafe { self.0.borrow_mut() }
    }
}
impl<T: ?Sized> Clone for Reference<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
///If you have a `Reference<Foo>` where `Foo` implements the `Bar` trait, you may end up wanting a
///`Reference<dyn Bar>`. To convert it, you would do this:
///```
///# use rrtk::*;
///struct Foo;
///impl Foo {
///    fn foo_func(&self) {}
///}
///trait Bar {
///    fn bar_func(&self) {}
///}
///impl Bar for Foo {}
///let ref_foo = static_reference!(Foo, Foo);
///ref_foo.borrow().foo_func();
///ref_foo.borrow().bar_func();
///let ref_dyn_bar = to_dyn!(Bar, ref_foo);
///ref_dyn_bar.borrow().bar_func();
///```
///
///The documentation shows `rrtk::to_dyn` and `rrtk::reference::to_dyn` separately. These are the
///same macro exported in two different places. These paths point to the same code in RRTK. Rust's
///scoping rules for macros are a bit odd, but you should be able to use `rrtk::to_dyn` and
///`rrtk::reference::to_dyn` interchangably.
#[macro_export]
macro_rules! to_dyn {
    ($trait_:path, $was:expr) => {{
        #[cfg(feature = "alloc")]
        extern crate alloc;
        #[allow(unreachable_patterns)]
        match $was.into_unsafe() {
            reference::ReferenceUnsafe::Ptr(ptr) => unsafe {
                Reference::from_ptr(ptr as *mut dyn $trait_)
            },
            #[cfg(feature = "alloc")]
            reference::ReferenceUnsafe::RcRefCell(rc_ref_cell) => Reference::from_rc_ref_cell(
                rc_ref_cell as alloc::rc::Rc<core::cell::RefCell<dyn $trait_>>,
            ),
            #[cfg(feature = "std")]
            reference::ReferenceUnsafe::PtrRwLock(ptr_rw_lock) => unsafe {
                Reference::from_ptr_rw_lock(ptr_rw_lock as *const std::sync::RwLock<dyn $trait_>)
            },
            _ => unimplemented!(),
        }
    }};
}
pub use to_dyn;
///Create a new `Rc<RefCell>` of something and return a `Reference` to it. Because of how `Rc`
///works, it won't be dropped until the last clone of the `Reference` is. This is reexported at the
///crate level.
#[cfg(feature = "alloc")]
pub fn rc_ref_cell_reference<T>(was: T) -> Reference<T> {
    Reference::from_rc_ref_cell(Rc::new(RefCell::new(was)))
}
///Create a static of something and return a `Ptr`-variant `Reference` to it. This contains a raw
///mutable pointer. It will never use-after-free because its target is static, but be careful if
///you're doing multiprocessing where multiple things could mutate it at once.
///
///The documentation shows `rrtk::static_reference` and `rrtk::reference::static_reference` separately. These are the
///same macro exported in two different places. These paths point to the same code in RRTK. Rust's
///scoping rules for macros are a bit odd, but you should be able to use `rrtk::static_reference` and
///`rrtk::reference::static_reference` interchangably.
#[macro_export]
macro_rules! static_reference {
    ($type_: ty, $was: expr) => {{
        static mut WAS: $type_ = $was;
        unsafe { Reference::from_ptr(core::ptr::addr_of_mut!(WAS)) }
    }};
}
pub use static_reference;
///Create a static `RwLock` of something and return a `PtrRwLock`-variant `Reference` to it.
///
///The documentation shows `rrtk::static_rw_lock_reference` and `rrtk::reference::static_rw_lock_reference` separately. These are the
///same macro exported in two different places. These paths point to the same code in RRTK. Rust's
///scoping rules for macros are a bit odd, but you should be able to use `rrtk::static_rw_lock_reference` and
///`rrtk::reference::static_rw_lock_reference` interchangably.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! static_rw_lock_reference {
    ($type_: ty, $was: expr) => {{
        static WAS: std::sync::RwLock<$type_> = std::sync::RwLock::new($was);
        unsafe { Reference::from_ptr_rw_lock(core::ptr::addr_of!(WAS)) }
    }};
}
#[cfg(feature = "std")]
pub use static_rw_lock_reference;
///Create a new static `Mutex` of something and return a `PtrMutex`-variant `Reference` to it.
///
///The documentation shows `rrtk::static_mutex_reference` and `rrtk::reference::static_mutex_reference` separately. These are the
///same macro exported in two different places. These paths point to the same code in RRTK. Rust's
///scoping rules for macros are a bit odd, but you should be able to use `rrtk::static_mutex_reference` and
///`rrtk::reference::static_mutex_reference` interchangably.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! static_mutex_reference {
    ($type_: ty, $was: expr) => {{
        static WAS: std::sync::Mutex<$type_> = std::sync::Mutex::new($was);
        unsafe { Reference::from_ptr_mutex(core::ptr::addr_of!(WAS)) }
    }};
}
///Create a new `Arc<RwLock>` of something and return a `Reference` to it. Because of how `Arc` and
///`Rc`, its single-threaded counterpart, work, it won't be dropped until the last clone of the
///`Reference` is. This is reexported at the crate level.
#[cfg(feature = "std")]
pub fn arc_rw_lock_reference<T>(was: T) -> Reference<T> {
    Reference::from_arc_rw_lock(Arc::new(RwLock::new(was)))
}
#[cfg(feature = "std")]
pub use static_mutex_reference;
///Create a new `Arc<Mutex>` of something and return a `Reference` to it. Because of how `Arc` and
///`Rc`, its single-threaded counterpart, work, it won't be dropped until the last clone of the
///`Reference` is. This is reexported at the crate level.
#[cfg(feature = "std")]
pub fn arc_mutex_reference<T>(was: T) -> Reference<T> {
    Reference::from_arc_mutex(Arc::new(Mutex::new(was)))
}
