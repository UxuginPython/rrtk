//!Contains `Reference`, a special enum vith variants for different kinds of references, and
//!related types. Everything here is reexported at the crate level.
use crate::*;
enum InternalBorrow<'a, T: ?Sized> {
    Ptr(*const T),
    RefCellRef(Ref<'a, T>),
}
impl<T: ?Sized> Deref for InternalBorrow<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr) => unsafe { &**ptr },
            Self::RefCellRef(refcell_ref) => refcell_ref,
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
    RefCellRefMut(RefMut<'a, T>),
}
impl<T: ?Sized> Deref for InternalBorrowMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Ptr(ptr) => unsafe { &**ptr },
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
        }
    }
}
impl<T: ?Sized> DerefMut for InternalBorrowMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Ptr(ptr) => unsafe { &mut **ptr },
            Self::RefCellRefMut(refcell_ref_mut) => refcell_ref_mut,
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
    RcRefCell(Rc<RefCell<T>>),
}
impl<T: ?Sized> InternalReference<T> {
    fn borrow(&self) -> InternalBorrow<'_, T> {
        match self {
            Self::Ptr(ptr) => InternalBorrow::Ptr(*ptr),
            Self::RcRefCell(rc_refcell) => InternalBorrow::RefCellRef(rc_refcell.borrow()),
        }
    }
    fn borrow_mut(&self) -> InternalBorrowMut<'_, T> {
        match self {
            Self::Ptr(ptr) => InternalBorrowMut::Ptr(*ptr),
            Self::RcRefCell(rc_refcell) => {
                InternalBorrowMut::RefCellRefMut(rc_refcell.borrow_mut())
            }
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
    pub fn from_rc_refcell(rc_refcell: Rc<RefCell<T>>) -> Self {
        Self(InternalReference::RcRefCell(rc_refcell))
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
