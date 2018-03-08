pub trait Trace {
  // Propagate `trace` calls through the objects graph
  // The goal is to call `trace` on `Gc` values (they have a special implementation to mark their
  // content)
  fn trace(&self);


  fn root(&self);

  // Propagate the objects graph to unroot `Gc` values that were on the stack but then moved inside
  // another `gc`
  fn unroot(&self);
}

impl Trace for u8 {
  fn trace(&self) {}

  fn root(&self) {}

  fn unroot(&self) {}
}

impl Trace for String {
  fn trace(&self) {}

  fn root(&self) {}

  fn unroot(&self) {}
}

impl<T: Trace> Trace for Option<T> {
  fn trace(&self) {
    match self {
      &Some(ref x) => x.trace(),
      &None => (),
    }
  }

  fn root(&self) {
    match self {
      &Some(ref x) => x.root(),
      &None => (),
    }
  }

  fn unroot(&self) {
    match self {
      &Some(ref x) => x.unroot(),
      &None => (),
    }
  }
}

impl<K: Eq + ::std::hash::Hash + Trace, V: Trace> Trace for ::std::collections::HashMap<K, V> {
  fn trace(&self) {
    for (k, v) in self.iter() {
      k.trace();
      v.trace();
    }
  }

  fn root(&self) {
    unimplemented!()
  }

  fn unroot(&self) {
    unimplemented!()
  }
}
