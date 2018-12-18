The crate `lib2` adds default `lib1` as dependencies for build, but enables
feature `feature1` of `lib1` for test, documentation and benchmarks. This is
a trick that `lib1` is added in both `dependencies` and `dev-dependencies` in
[lib2/Cargo.toml](lib2/Cargo.toml).

However, the cargo option `--all` leads to different behaviours. See how the
[travis test](https://travis-ci.com/doitian/cargo-test) failed.

## Test

In test, the `feature1` should be enabled, so `lib2::foo()` must return 1, as
asserted in the test case in [src/main.rs](src/main.rs).

However, `cargo test --all` passes the test, while `cargo test` fails.

## Build

In build generated binary, the `feature1` should be disabled, so `lib2::foo()`
must return 0 and the binary should print `foo 0`.

However, the result also depends on whether the `--all` is added:

```
cargo build &>/dev/null && target/debug/foo
# foo 0
cargo build --all &>/dev/null && target/debug/foo
# foo 1
cargo build --release &>/dev/null && target/release/foo
# foo 0
cargo build --release --all &>/dev/null && target/release/foo
# foo 1
```

## One Thing More

If we change the top Cargo.toml as in #1，`feature1` is always enabled.
