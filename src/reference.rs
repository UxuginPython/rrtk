//!Contains `Reference`, a special enum vith variants for different kinds of references, and
//!related types. Everything here is reexported at the crate level.
use crate::*;
#[cfg(feature = "std")]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
enum InternalBorrow<'a, T: ?Sized> {
    Ptr(*const T),
    #[cfg(feature = "alloc")]
    RefCellRef(Ref<'a, T>),
    #[cfg(feature = "std")]
    RwLockReadGuard(RwLockReadGuard<'a, T>),
}
impl<T: ?Sized> Deref for InternalBorrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRef(refcell_ref) => refcell_ref,
            #[cfg(feature = "std")]
            Self::RwLockReadGuard(rw_lock_read_guard) => rw_lock_read_guard,
        }
    }
}
///An immutable borrow of an RRTK `Reference`, similar to `Ref` for a `RefCell`.
pub struct Borrow<'a, T: ?Sized>(InternalBorrow<'a, T>);
impl<T: ?Sized> Deref for Borrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
enum InternalBorrowMut<'a, T: ?Sized> {
    Ptr(*mut T),
    #[cfg(feature = "alloc")]
    RefCellRefMut(RefMut<'a, T>),
    #[cfg(feature = "std")]
    RwLockWriteGuard(RwLockWriteGuard<'a, T>),
}
impl<T: ?Sized> Deref for InternalBorrowMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr) => unsafe { &**ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
        }
    }
}
impl<T: ?Sized> DerefMut for InternalBorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Ptr(ptr) => unsafe { &mut **ptr },
            #[cfg(feature = "alloc")]
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
            #[cfg(feature = "std")]
            Self::RwLockWriteGuard(rw_lock_write_guard) => rw_lock_write_guard,
        }
    }
}
///A mutable borrow of an RRTK `Reference`, similar to `RefMut` for a `RefCell`.
pub struct BorrowMut<'a, T: ?Sized>(InternalBorrowMut<'a, T>);
impl<T: ?Sized> Deref for BorrowMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T: ?Sized> DerefMut for BorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
enum InternalReference<T: ?Sized> {
    Ptr(*mut T),
    #[cfg(feature = "alloc")]
    RcRefCell(Rc<RefCell<T>>),
    #[cfg(feature = "std")]
    PtrRwLock(*const RwLock<T>),
}
impl<T: ?Sized> InternalReference<T> {
    fn borrow(&self) -> InternalBorrow<'_, T> {
        match self {
            Self::Ptr(ptr) => InternalBorrow::Ptr(*ptr),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_refcell) => InternalBorrow::RefCellRef(rc_refcell.borrow()),
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                InternalBorrow::RwLockReadGuard(
                    (**ptr_rw_lock)
                        .read()
                        .expect("RRTK Reference borrow failed to get RwLock read lock"),
                )
            },
        }
    }
    fn borrow_mut(&self) -> InternalBorrowMut<'_, T> {
        match self {
            Self::Ptr(ptr) => InternalBorrowMut::Ptr(*ptr),
            #[cfg(feature = "alloc")]
            Self::RcRefCell(rc_refcell) => {
                InternalBorrowMut::RefCellRefMut(rc_refcell.borrow_mut())
            }
            #[cfg(feature = "std")]
            Self::PtrRwLock(ptr_rw_lock) => unsafe {
                InternalBorrowMut::RwLockWriteGuard(
                    (**ptr_rw_lock)
                        .write()
                        .expect("RRTK Reference mutable borrow failed to get RwLock write lock"),
                )
            },
        }
    }
}
///A special enum with variants for different kinds of references depending on your platform and
///code structure. (Some variants are alloc- or std-only.)
pub struct Reference<T: ?Sized>(InternalReference<T>);
impl<T: ?Sized> Reference<T> {
    ///Create a `Reference` from a raw mutable pointer. Good if you're not multithreading and you
    ///want to avoid dynamic allocation. Making the object static is strongly recommended if you
    ///use this.
    pub unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self(InternalReference::Ptr(ptr))
    }
    ///Create a `Reference` from an `Rc<RefCell<T>>`.
    #[cfg(feature = "alloc")]
    pub fn from_rc_refcell(rc_refcell: Rc<RefCell<T>>) -> Self {
        Self(InternalReference::RcRefCell(rc_refcell))
    }
    ///Create a `Reference` from a `*const RwLock<T>`.
    #[cfg(feature = "std")]
    pub unsafe fn from_rwlock_ptr(ptr_rwlock: *const RwLock<T>) -> Self {
        Self(InternalReference::PtrRwLock(ptr_rwlock))
    }
    ///Immutably borrow a `Reference`, similarly to how you would with a `RefCell`.
    pub fn borrow(&self) -> Borrow<'_, T> {
        Borrow(self.0.borrow())
    }
    ///Mutably borrow a `Reference`, similarly to how you would with a `RefCell`.
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        BorrowMut(self.0.borrow_mut())
    }
}
