use {Gc, GcRefCell, GcScope, Trace};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct RefNamedObject<'n> {
  pub name: &'n str,
}

unsafe impl<'a> Trace for RefNamedObject<'a> {
  unsafe fn mark(&self) {}
  unsafe fn root(&self) {}
  unsafe fn unroot(&self) {}
}

#[derive(Debug)]
pub struct NamedObject {
  pub name: String,
}

unsafe impl Trace for NamedObject {
  unsafe fn mark(&self) {}
  unsafe fn root(&self) {}
  unsafe fn unroot(&self) {}
}

#[derive(Debug)]
pub struct CircularNamedObject<'a> {
  pub name: String,
  pub other: Option<Gc<'a, GcRefCell<CircularNamedObject<'a>>>>,
}

unsafe impl<'a> Trace for CircularNamedObject<'a> {
  unsafe fn mark(&self) {
    self.other.mark();
  }
  unsafe fn root(&self) {
    self.other.root();
  }
  unsafe fn unroot(&self) {
    self.other.unroot();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_gc_simple() {
  let scope: GcScope = GcScope::new();
  scope.alloc(String::from("foo")).unwrap();
}

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
