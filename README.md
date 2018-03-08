# Scoped Garbage collection

This crate lets you create a scope where you can use garbage collection.
It grants you higher control over the lifetime of the data: it is dropped at the end of
the scope.

The implementation and design are heavily inspired by [rust-gc](https://github.com/Manishearth/rust-gc).
