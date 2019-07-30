#![feature(dropck_eyepatch)]

/// This module lets you create garbage-collected scopes
///
/// ```compile_fail
/// use scoped_gc::{Gc, GcScope, Trace};
///
/// pub struct NamedObject {
///   pub name: String,
/// }
///
/// unsafe impl Trace for NamedObject {
///   unsafe fn mark(&self) {}
///   unsafe fn root(&self) {}
///   unsafe fn unroot(&self) {}
/// }
///
/// fn main() {
///   let message: Gc<NamedObject>;
///   {
///     let scope: GcScope = GcScope::new();
///     message = scope.alloc(NamedObject { name: String::from("Hello, World!") }).unwrap();
///   }
///   println!("{}", message.name);
/// }
/// ```
///
/// ```compile_fail
/// use scoped_gc::{Gc, GcScope, Trace};
///
/// pub struct RefNamedObject<'a> {
///   pub name: &'a str,
/// }
///
/// unsafe impl<'a> Trace for RefNamedObject<'a> {
///   unsafe fn mark(&self) {}
///   unsafe fn root(&self) {}
///   unsafe fn unroot(&self) {}
/// }
///
/// fn main() {
///   let scope: GcScope = GcScope::new();
///   let message: Gc<RefNamedObject>;
///   {
///     let hello_world: String = String::from("Hello, World!");
///     message = scope.alloc(RefNamedObject { name: &hello_world }).unwrap();
///   }
/// }
/// ```
///
/// ```compile_fail
/// // Check that the drop order between the GC scope and values is enforced.
///
/// use scoped_gc::{Gc, GcScope, Trace};
///
/// pub struct NamedObject {
///   pub name: String,
/// }
///
/// unsafe impl Trace for NamedObject {
///   unsafe fn mark(&self) {}
///   unsafe fn root(&self) {}
///   unsafe fn unroot(&self) {}
/// }
///
/// fn main() {
///   let mut stack: Vec<Gc<NamedObject>> = Vec::new();
///   let scope: GcScope = GcScope::new();
///   stack.push(scope.alloc(NamedObject { name: String::from("Hello, World!") }).unwrap())
///   // `scope` is dropped first and frees the `NamedObject`
///   // `stack` is dropped second, but it contains a `Gc`!
///   // It will try to decrement the root count of an already freed value
/// }
/// ```

mod gc;
mod gc_alloc_err;
mod gc_box;
mod gc_ref_cell;
mod gc_scope;
mod trace;

#[cfg(test)]
mod test;

pub use gc::Gc;
pub use gc_alloc_err::GcAllocErr;
pub use gc_ref_cell::{GcRef, GcRefCell, GcRefMut};
pub use gc_scope::GcScope;
pub use trace::Trace;
