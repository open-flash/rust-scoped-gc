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

unsafe impl Trace for u8 {
  unsafe fn mark(&self) {}

  unsafe fn root(&self) {}

  unsafe fn unroot(&self) {}
}

unsafe impl Trace for String {
  unsafe fn mark(&self) {}

  unsafe fn root(&self) {}

  unsafe fn unroot(&self) {}
}

unsafe impl<T: Trace> Trace for Option<T> {
  unsafe fn mark(&self) {
    match self {
      &Some(ref x) => x.mark(),
      &None => (),
    }
  }

  unsafe fn root(&self) {
    match self {
      &Some(ref x) => x.root(),
      &None => (),
    }
  }

  unsafe fn unroot(&self) {
    match self {
      &Some(ref x) => x.unroot(),
      &None => (),
    }
  }
}

unsafe impl<K: Eq + ::std::hash::Hash + Trace, V: Trace> Trace for ::std::collections::HashMap<K, V> {
  unsafe fn mark(&self) {
    for (k, v) in self.iter() {
      k.mark();
      v.mark();
    }
  }

  unsafe fn root(&self) {
    for (k, v) in self.iter() {
      k.root();
      v.root();
    }
  }

  unsafe fn unroot(&self) {
    for (k, v) in self.iter() {
      k.unroot();
      v.unroot();
    }
  }
}

unsafe impl<T: Trace> Trace for ::std::vec::Vec<T> {
  unsafe fn mark(&self) {
    for item in self.iter() {
      item.mark();
    }
  }

  unsafe fn root(&self) {
    for item in self.iter() {
      item.root();
    }
  }

  unsafe fn unroot(&self) {
    for item in self.iter() {
      item.unroot();
    }
  }
}
