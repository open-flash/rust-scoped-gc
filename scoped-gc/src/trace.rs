/// Used to propagate signals across the objects graph of values managed by the garbage collector.
///
/// This trait is `unsafe` because an invalid implementations may cause dangling pointers.
/// For example, if `trace` is not propagated to a reachable `Gc` pointers then its value may be
/// freed. The next dereference of this `Gc` causes then an error.
///
/// You should never initiate the traversal of the object graph: it is the role of the library.
/// Propagating a signal at the wrong time may cause an invalid state leading to dangling pointers.
pub unsafe trait Trace {
  /// Propagates the `mark` signal across the objects graph.
  ///
  /// This signal is used during the first phase of garbage collection to mark the values that are
  /// still reachable.
  ///
  /// The signal is initiated at the rooted values and propagated to reach the adjacent `Gc`
  /// pointers. If their value is not already marked they mark it and propagate the signal further
  /// in the graph.
  unsafe fn mark(&self);

  /// Propagates the `root` signal across the objects graph.
  ///
  /// This signal is used to signal that there is a new root pointing to a managed value. This
  /// prevents this value (and any other value reachable from it) from being collected.
  ///
  /// This is initiated when creating a new `Gc` pointer or mutably borrowing the value inside a
  /// `GcRefCell`.
  unsafe fn root(&self);

  /// Propagates the `unroot` signal across the objects graph.
  ///
  /// This signal is used to signal the destruction of a root pointing to a managed value. If this
  /// was the last root, the value is not longer used to initiate `mark` signals: if there are no
  /// other rooted values that can reach it, it becomes eligible for garbage collection.
  ///
  /// This is initiated when a `Gc` pointer or mutably borrowing the value inside a
  /// `GcRefCell`.
  unsafe fn unroot(&self);
}

/// This macro rule implements `Trace` with empty functions.
///
/// Use this for types that can't contain other `Trace` types.
#[macro_export]
macro_rules! unsafe_empty_trace {
  ($T: ty) => {
    unsafe impl Trace for $T {
      #[inline]
      unsafe fn mark(&self) {}
      #[inline]
      unsafe fn root(&self) {}
      #[inline]
      unsafe fn unroot(&self) {}
    }
  }
}

unsafe_empty_trace!(());
unsafe_empty_trace!(bool);
unsafe_empty_trace!(u8);
unsafe_empty_trace!(u16);
unsafe_empty_trace!(u32);
unsafe_empty_trace!(u64);
unsafe_empty_trace!(usize);
unsafe_empty_trace!(i8);
unsafe_empty_trace!(i16);
unsafe_empty_trace!(i32);
unsafe_empty_trace!(i64);
unsafe_empty_trace!(isize);
unsafe_empty_trace!(f32);
unsafe_empty_trace!(f64);
unsafe_empty_trace!(char);
unsafe_empty_trace!(String);
unsafe_empty_trace!(::std::path::Path);
unsafe_empty_trace!(::std::path::PathBuf);
unsafe_empty_trace!(::std::sync::atomic::AtomicBool);
unsafe_empty_trace!(::std::sync::atomic::AtomicIsize);
unsafe_empty_trace!(::std::sync::atomic::AtomicUsize);

#[macro_export]
macro_rules! unsafe_custom_trace {
  ($this:ident, $body:expr) => {
    #[inline]
    unsafe fn mark(&self) {
      #[inline]
      unsafe fn trace<T: $crate::Trace>(it: &T) { $crate::Trace::mark(it) }
      let $this = self;
      $body
    }
    #[inline]
    unsafe fn root(&self) {
      #[inline]
      unsafe fn trace<T: $crate::Trace>(it: &T) { $crate::Trace::root(it) }
      let $this = self;
      $body
    }
    #[inline]
    unsafe fn unroot(&self) {
      #[inline]
      unsafe fn trace<T: $crate::Trace>(it: &T) { $crate::Trace::unroot(it) }
      let $this = self;
      $body
    }
  }
}

unsafe impl<T: Trace> Trace for Box<T> {
  unsafe_custom_trace!(this, {
    trace(&**this)
  });
}

unsafe impl<T: Trace> Trace for Option<T> {
  unsafe_custom_trace!(this, {
    if let Some(ref x) = *this { trace(x) }
  });
}

unsafe impl<T: Trace, E: Trace> Trace for Result<T, E> {
  unsafe_custom_trace!(this, {
    match *this {
      Ok(ref r) => trace(r),
      Err(ref e) => trace(e),
    }
  });
}

unsafe impl<T: Trace> Trace for Vec<T> {
  unsafe_custom_trace!(this, {
    for item in this {
      trace(item)
    }
  });
}

unsafe impl<K: Eq + ::std::hash::Hash + Trace, V: Trace> Trace for ::std::collections::HashMap<K, V> {
  unsafe_custom_trace!(this, {
    for (k, v) in this.iter() {
      trace(k);
      trace(v);
    }
  });
}
