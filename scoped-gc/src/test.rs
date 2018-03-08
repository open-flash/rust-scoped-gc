use {Gc, GcRefCell, GcScope, Trace};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RefNamedObject<'n> {
  pub name: &'n str,
}

impl<'a> Trace for RefNamedObject<'a> {
  fn trace(&self) {}
  fn root(&self) {}
  fn unroot(&self) {}
}

#[derive(Debug)]
pub struct NamedObject {
  pub name: String,
}

impl Trace for NamedObject {
  fn trace(&self) {}
  fn root(&self) {}
  fn unroot(&self) {}
}

#[derive(Debug)]
pub struct CircularNamedObject<'a> {
  pub name: String,
  pub other: Option<Gc<'a, GcRefCell<CircularNamedObject<'a>>>>,
}

impl<'a> Trace for CircularNamedObject<'a> {
  fn trace(&self) {
    self.other.trace();
  }
  fn root(&self) {
    self.other.root();
  }
  fn unroot(&self) {
    self.other.unroot();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_gc() {
  let scope: GcScope = GcScope::new();
  let n1: Gc<NamedObject>;
  {
    let n2: Gc<NamedObject> = scope.alloc(NamedObject { name: String::from("Hello, World!") }).unwrap();
    n1 = Gc::clone(&n2);
  }
  assert_eq!(n1.name, String::from("Hello, World!"));
}

#[test]
fn test_gc_ref() {
  let a: String = String::from("Hello, World!");
  {
    let scope: GcScope = GcScope::new();
    let n: Gc<RefNamedObject> = scope.alloc(RefNamedObject { name: &a }).unwrap();
    assert_eq!(n.name, String::from("Hello, World!"));
  }
}

#[test]
fn test_gc_circular() {
  let scope: GcScope = GcScope::new();
  let n1 = scope.alloc(GcRefCell::new(CircularNamedObject { name: String::from("n1"), other: None })).unwrap();
  let n2 = scope.alloc(GcRefCell::new(CircularNamedObject { name: String::from("n2"), other: None })).unwrap();
  n1.borrow_mut().other = Some(Gc::clone(&n2));
  n2.borrow_mut().other = Some(Gc::clone(&n1));
}
