use ::std::cell::{Cell, RefCell};
use ::std::mem::{size_of, size_of_val};
use ::std::ptr::NonNull;
use gc::Gc;
use gc_alloc_err::GcAllocErr;
use gc_box::GcBox;
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
  pub fn alloc<T: Trace + 'gc>(&'gc self, value: T) -> Result<Gc<'gc, T>, GcAllocErr> {
    unsafe { value.unroot() }
    self.state.borrow_mut()
      .alloc(value)
      .map(|ptr| Gc::new(ptr))
  }

  pub fn collect_garbage(&self) {
    self.state.borrow_mut().collect_garbage()
  }
}

#[derive(Debug)]
struct GcState<'gc> {
  pub(crate) allocated_bytes: usize,
  //  threshold: usize,
  // Linked-list of boxes
  pub(crate) boxes: Option<NonNull<GcBox<'gc, Trace>>>,
}

impl<'gc> GcState<'gc> {
  pub(crate) fn new() -> GcState<'gc> {
    GcState {
      allocated_bytes: 0,
      boxes: None,
    }
  }

  // Allocates GC-managed memory for T
  pub(crate) fn alloc<T: Trace + 'gc>(&mut self, value: T) -> Result<NonNull<GcBox<'gc, T>>, GcAllocErr> {
    // into_raw -> mem::forget, so we need to make sure we deallocate it ourselve
    let gc_box_ptr: *mut GcBox<T> = Box::into_raw(Box::new(GcBox {
      roots: Cell::new(1),
      marked: Cell::new(false),
      next: self.boxes,
      value: value,
    }));
    self.allocated_bytes += size_of::<GcBox<T>>();
    // We know that `gc_box` is not null so we can use `new_unchecked`
    let box_ptr: NonNull<GcBox<T>> = unsafe { NonNull::new_unchecked(gc_box_ptr) };
    self.boxes = Some(box_ptr);
    Ok(unsafe { NonNull::new_unchecked(gc_box_ptr) })
  }

  pub(crate) fn collect_garbage(&mut self) {
    {
      // Mark
      let mut next_gc_box_ptr = self.boxes;
      while let Some(gc_box_ptr) = next_gc_box_ptr {
        let gc_box: &GcBox<Trace> = unsafe { gc_box_ptr.as_ref() };
        if gc_box.roots.get() > 0 {
          gc_box.mark_box();
        }
        next_gc_box_ptr = gc_box.next;
      }
    }

    let mut unmarked: Vec<*mut GcBox<Trace>> = Vec::new();
    unsafe {
      // Collect
      let mut next_gc_box_ptr_ref = &mut self.boxes;
      while let Some(gc_box_ptr) = *next_gc_box_ptr_ref {
        let gc_box_ptr = gc_box_ptr.as_ptr();
        if (*gc_box_ptr).marked.get() {
          (*gc_box_ptr).marked.set(false);
          next_gc_box_ptr_ref = &mut (*gc_box_ptr).next;
        } else {
          *next_gc_box_ptr_ref = (*gc_box_ptr).next;
          unmarked.push(gc_box_ptr);
        }
      }
    }

    for gc_box_ptr in unmarked.iter() {
      let gc_box = unsafe { Box::from_raw(*gc_box_ptr) };
      self.allocated_bytes = self.allocated_bytes.checked_sub(size_of_val::<GcBox<_>>(gc_box.as_ref())).unwrap()
      // Implicitly drops `gc_box` and frees the associated memory
    }
  }
}

unsafe impl<#[may_dangle] 'gc> Drop for GcState<'gc> {
  fn drop(&mut self) {
    let mut cur_box = self.boxes;
    while let Some(gc_box_ptr) = cur_box {
      let gc_box = unsafe { Box::from_raw(gc_box_ptr.as_ptr()) };
      cur_box = (*gc_box).next;
      // Implicitly drops `gc_box` and frees the associated memory
    }
  }
}
