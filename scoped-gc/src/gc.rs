use ::std::cell::Cell;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use gc_box::GcBox;
use trace::Trace;

#[derive(Debug)]
pub struct Gc<'gc, T: Trace + 'gc> {
  pub ptr: NonNull<GcBox<'gc, T>>,
  pub rooted: Cell<bool>,
}

trait GcBoxPtr<'gc, T: Trace + 'gc> {
  fn inner(&self) -> &GcBox<T>;
}

impl<'gc, T: Trace + 'gc> GcBoxPtr<'gc, T> for Gc<'gc, T> {
  fn inner(&self) -> &GcBox<T> {
    unsafe { self.ptr.as_ref() }
  }
}

unsafe impl<'gc, T: Trace> Trace for Gc<'gc, T> {
  unsafe fn mark(&self) {
    self.inner().mark_box();
  }

  unsafe fn root(&self) {
    debug_assert!(!self.rooted.get());
    self.inner().inc_roots();
    self.rooted.set(true);
  }

  unsafe fn unroot(&self) {
    debug_assert!(self.rooted.get());
    self.inner().dec_roots();
    self.rooted.set(false);
  }
}

impl<'gc, T: Trace> Deref for Gc<'gc, T> {
  type Target = T;

  fn deref(&self) -> &T {
    &self.inner().value
  }
}


impl<'gc, T: Trace> Drop for Gc<'gc, T> {
  fn drop(&mut self) {
    if self.rooted.get() {
      self.inner().dec_roots();
    }
  }
}

impl<'gc, T: Trace + 'gc> Clone for Gc<'gc, T> {
  fn clone(&self) -> Gc<'gc, T> {
    self.inner().inc_roots();
    Gc { ptr: self.ptr, rooted: Cell::new(true) }
  }
}
