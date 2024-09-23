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
pub struct Reference<T: ?Sized>(InternalReference<T>);
impl<T: ?Sized> Reference<T> {
    pub unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self(InternalReference::Ptr(ptr))
    }
    pub fn from_rc_refcell(rc_refcell: Rc<RefCell<T>>) -> Self {
        Self(InternalReference::RcRefCell(rc_refcell))
    }
    pub fn borrow(&self) -> Borrow<'_, T> {
        Borrow(self.0.borrow())
    }
    pub fn borrow_mut(&self) -> BorrowMut<'_, T> {
        BorrowMut(self.0.borrow_mut())
    }
}
