extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;
#[macro_use]
extern crate synstructure;

decl_derive!([Trace] => derive_trace);

fn derive_trace(s: synstructure::Structure) -> quote::Tokens {
  let trace_body = s.each(|bi| quote!(mark(#bi)));

  let trace_impl = s.unsafe_bound_impl("::scoped_gc::Trace", quote! {
    #[inline] unsafe fn mark(&self) {
      #[allow(dead_code)]
      #[inline]
      unsafe fn mark<T: ::scoped_gc::Trace>(it: &T) {
        ::gc::Trace::mark(it);
      }
      match *self { #trace_body }
    }
    #[inline] unsafe fn root(&self) {
      #[allow(dead_code)]
      #[inline]
      unsafe fn mark<T: ::scoped_gc::Trace>(it: &T) {
        ::gc::Trace::root(it);
      }
      match *self { #trace_body }
    }
    #[inline] unsafe fn unroot(&self) {
      #[allow(dead_code)]
      #[inline]
      unsafe fn mark<T: ::scoped_gc::Trace>(it: &T) {
        ::gc::Trace::unroot(it);
      }
      match *self { #trace_body }
    }
  });

  quote! { #trace_impl }
}
