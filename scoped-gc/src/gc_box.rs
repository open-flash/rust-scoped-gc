use ::std::cell::Cell;
use ::std::ptr::NonNull;
use trace::Trace;

/// Internal struct containing the values allocated by the garbage collector, with their metadata.
///
/// This struct is heap-allocated during `GcScope::alloc`.
#[derive(Debug)]
pub(crate) struct GcBox<'gc, T: Trace + ? Sized + 'gc> {
  /// A counter for the `Gc` pointers or `GcRefMut` acting as roots for this value.
  ///
  /// Boxes with a non-zero root count act as starting points for the "mark" phase of the
  /// garbage collector.
  pub(crate) roots: Cell<usize>,

  /// A boolean used during the "mark" phase of the garbage-collection to signal that this box is
  /// still reachable.
  pub(crate) marked: Cell<bool>,

  /// A fat pointer (trait object) to the next `GcBox` if any.
  pub(crate) next: Option<NonNull<GcBox<'gc, Trace>>>,

  /// The value the user allocated.
  pub(crate) value: T,
}

impl<'gc, T: Trace + ? Sized + 'gc> GcBox<'gc, T> {
  pub fn mark_box(&self) {
    if !self.marked.get() {
      self.marked.set(true);
      unsafe { self.value.mark() }
    }
  }

  pub fn inc_roots(&self) {
    self.roots.set(self.roots.get().checked_add(1).unwrap())
  }

  pub fn dec_roots(&self) {
    self.roots.set(self.roots.get().checked_sub(1).unwrap())
  }
}
