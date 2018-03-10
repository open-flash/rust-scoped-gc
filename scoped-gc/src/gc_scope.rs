use ::std::cell::RefCell;
use gc::Gc;
use gc_alloc_err::GcAllocErr;
use gc_state::GcState;
use trace::Trace;

/// Defines a scope for garbage collection.
///
/// It lets you allocate garbage-collected values. They can have cycles. Their reachability is
/// tracked so they can be deallocated once unreachable.
/// All the values are deallocated once the scope is dropped.
#[derive(Debug)]
pub struct GcScope<'gc> {
  state: RefCell<GcState<'gc>>,
}

impl<'gc> GcScope<'gc> {
  pub fn new() -> GcScope<'gc> {
    GcScope { state: RefCell::new(GcState::new()) }
  }

  /// Allocates `value` in this garbage-collected scope and returns a `Gc` smart pointer to it.
  pub fn alloc<T: Trace + 'gc>(&self, value: T) -> Result<Gc<T>, GcAllocErr> {
    unsafe { value.unroot() }
    self.state.borrow_mut()
      .alloc(value)
      .map(|ptr| Gc::new(ptr))
  }

  pub fn collect_garbage(&self) {
    self.state.borrow_mut().collect_garbage()
  }
}
