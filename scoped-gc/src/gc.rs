use ::std::cell::Cell;
use ::std::ops::Deref;
use ::std::ptr::NonNull;
use gc_box::GcBox;
use trace::Trace;

/// A smart pointer to a value managed by a garbage-collector
///
/// This pointer can be created either by allocating a new garbage-collected value using the
/// `GcScope::alloc` method or by cloning an existing `Gc` pointer with `Gc::clone`.
/// This pointer can only be used during the lifetime of the corresponding garbage-collected
/// scope (represented by the lifetime `'gc`).
///
/// By default, a `Gc` pointer acts as a root for its value: as long as the `Gc` is on the stack
/// its value is kept alive.
/// A `Gc` is unrooted only when it is moved (transitively) inside another `Gc`.
#[derive(Debug)]
pub struct Gc<'gc, T: Trace + 'gc> {
  ptr: NonNull<GcBox<'gc, T>>,
  rooted: Cell<bool>,
}

impl<'gc, T: Trace + 'gc> Gc<'gc, T> {
  pub(crate) fn new(ptr: NonNull<GcBox<'gc, T>>) -> Gc<'gc, T> {
    Gc { ptr, rooted: Cell::new(true) }
  }
}

/// An internal trait to get a reference for the box containing a garbage-collected value.
trait GcBoxPtr<'gc, T: Trace + 'gc> {
  fn inner(&self) -> &GcBox<T>;
}

impl<'gc, T: Trace + 'gc> GcBoxPtr<'gc, T> for Gc<'gc, T> {
  fn inner(&self) -> &GcBox<T> {
    unsafe { self.ptr.as_ref() }
  }
}

unsafe impl<'gc, T: Trace> Trace for Gc<'gc, T> {
  /// Marks the value in the `GcBox` as reachable.
  ///
  /// The `mark` signal will be propagated further in the object graph unless the box was already
  /// marked (to avoid infinite loops on cycles, or redundant traversals).
  unsafe fn mark(&self) {
    self.inner().mark_box();
  }

  /// Tags this `Gc` pointer as a root for its value.
  unsafe fn root(&self) {
    debug_assert!(!self.rooted.get());
    self.inner().inc_roots();
    self.rooted.set(true);
  }

  /// Untags this `Gc` pointer as a root for its value.
  unsafe fn unroot(&self) {
    debug_assert!(self.rooted.get());
    self.inner().dec_roots();
    self.rooted.set(false);
  }
}

/// The `Deref` implementation allows to use the value's methods directly on the `Gc` pointer.
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

/// Creates a new `Gc` pointer for the same value.
///
/// It is recommended to use it as `Gc::clone(&gc)` instead of `gc.clone()` to avoid confusion
/// with the `clone` method of the inner value.
impl<'gc, T: Trace + 'gc> Clone for Gc<'gc, T> {
  fn clone(&self) -> Gc<'gc, T> {
    self.inner().inc_roots();
    Gc { ptr: self.ptr, rooted: Cell::new(true) }
  }
}
