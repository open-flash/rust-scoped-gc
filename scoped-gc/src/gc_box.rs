use ::std::cell::Cell;
use ::std::ptr::NonNull;
use trace::Trace;

// Private: keeps track of the roots and marked state
#[derive(Debug)]
pub struct GcBox<'gc, T: Trace + ? Sized + 'gc> {
  // 8 bytes
  pub roots: Cell<usize>,
  // 1 byte
  pub marked: Cell<bool>,
  // 16 bytes
  pub next: Option<NonNull<GcBox<'gc, Trace>>>,
  pub value: T,
}

impl<'gc, T: Trace + ? Sized + 'gc> GcBox<'gc, T> {
  pub fn mark_box(&self) {
    if !self.marked.get() {
      self.marked.set(true);
      self.value.trace()
    }
  }

  pub fn inc_roots(&self) {
    self.roots.set(self.roots.get().checked_add(1).unwrap())
  }

  pub fn dec_roots(&self) {
    self.roots.set(self.roots.get().checked_sub(1).unwrap())
  }
}
