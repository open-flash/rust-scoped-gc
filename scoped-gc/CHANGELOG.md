## 0.1.5 (2019-08-07)

- **[Feature]** Implement `Trace` for `::std::ops::Range`.
- **[Feature]** Implement `Trace` for `str` and `&str`.
- **[Fix]** Add explicit `dyn` to trait objects.

## 0.1.4 (2019-07-30)

- **[Fix]** **Fix use-after-free** caused by invalid lifetime constraint.

## 0.1.3 (2019-03-06)

- **[Fix]** Fix lifetime elision error on `GcScope::alloc` ([#3](https://github.com/open-flash/rust-scoped-gc/pull/3))
- **[Fix]** Remove `generic_param_attrs` feature, stable since Rust `1.27.0`.
- **[Internal]**: Add Travis CI integration.

## 0.1.2 (2018-11-26)

- **[Fix]**: Add `Trace` impl for `BTreeMap`
- **[Internal]**: Add `CHANGELOG.md`
