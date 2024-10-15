// SPDX-License-Identifier: BSD-3-Clause
// Copyright 2024 UxuginPython
use rrtk::*;
#[test]
fn ptr() {
    let x = static_reference!(u8, 5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
#[test]
#[cfg(feature = "alloc")]
fn rc_refcell() {
    let x = rc_ref_cell_reference(5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
#[test]
#[cfg(feature = "std")]
fn ptr_rw_lock() {
    let x = static_rw_lock_reference!(u8, 5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
#[test]
#[cfg(feature = "std")]
fn ptr_mutex() {
    let x = static_mutex_reference!(u8, 5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
#[test]
#[cfg(feature = "std")]
fn arc_rw_lock() {
    let x = arc_rw_lock_reference(5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
#[test]
#[cfg(feature = "std")]
fn arc_mutex() {
    let x = arc_mutex_reference(5);
    {
        let x_borrow = x.borrow();
        assert_eq!(*x_borrow, 5);
    }
    let mut x_borrow_mut = x.borrow_mut();
    assert_eq!(*x_borrow_mut, 5);
    *x_borrow_mut += 1;
    assert_eq!(*x_borrow_mut, 6);
}
