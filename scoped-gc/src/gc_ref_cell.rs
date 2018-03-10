use ::std::cell::{Cell, Ref, RefCell, RefMut};
use ::std::ops::{Deref, DerefMut};
use super::trace::Trace;

#[derive(Debug)]
pub struct GcRefCell<T: Trace> {
  rooted: Cell<bool>,
  ref_cell: RefCell<T>,
}

impl<T: Trace> GcRefCell<T> {
  pub fn new(value: T) -> GcRefCell<T> {
    GcRefCell {
      rooted: Cell::new(true),
      ref_cell: RefCell::new(value),
    }
  }

  pub fn borrow(&self) -> GcRef<T> {
    GcRef { _ref: self.ref_cell.borrow() }
  }

  pub fn borrow_mut(&self) -> GcRefMut<T> {
    // Root the content of the cell for the duration of the mutable borrow, this will be restored
    // once `GcRefMut` is dropped.
    if !self.rooted.get() {
      unsafe { self.ref_cell.borrow().root(); }
    }
    GcRefMut { rooted: &self.rooted, _ref: self.ref_cell.borrow_mut() }
  }
}

unsafe impl<T: Trace> Trace for GcRefCell<T> {
  unsafe fn mark(&self) {
    // If we can't borrow, it means that there is an active RefMut and the value is rooted
    // (no need to trace)
    match self.ref_cell.try_borrow() {
      Ok(ref value) => value.mark(),
      Err(_) => (),
    }
  }

  unsafe fn root(&self) {
    assert!(!self.rooted.get());
    self.rooted.set(true);
    match self.ref_cell.try_borrow() {
      Ok(ref value) => value.root(),
      Err(_) => (),
    }
  }

  unsafe fn unroot(&self) {
    assert!(self.rooted.get());
    self.rooted.set(false);
    match self.ref_cell.try_borrow() {
      Ok(ref value) => value.unroot(),
      Err(_) => (),
    }
  }
}

pub struct GcRef<'a, T: Trace + 'a> {
  _ref: Ref<'a, T>,
}

impl<'a, T: Trace + 'a> Deref for GcRef<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self._ref.deref()
  }
}

pub struct GcRefMut<'a, T: Trace + 'a> {
  rooted: &'a Cell<bool>,
  _ref: RefMut<'a, T>,
}

impl<'a, T: Trace + 'a> Deref for GcRefMut<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self._ref.deref()
  }
}

impl<'a, T: Trace + 'a> DerefMut for GcRefMut<'a, T> {
  fn deref_mut(&mut self) -> &mut T {
    self._ref.deref_mut()
  }
}

impl<'a, T: Trace + 'a> Drop for GcRefMut<'a, T> {
  fn drop(&mut self) {
    // Restore the `rooted state` of the inner value before the call to `borrow_mut`
    if !self.rooted.get() {
      unsafe { self._ref.unroot(); }
    }
  }
}
