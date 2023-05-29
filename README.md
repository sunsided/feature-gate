# Simple documented feature gates

This crates provides the `feature_gate` and `feature_gate_ex`
macros for simple `#[cfg(feature = "...")]` macros that are properly
documented on docs.rs.

## Stable Rust

Note that for it to work properly on stable Rust, the following needs to be
added to `Cargo.toml` for the time being (see [Metadata for custom builds](https://docs.rs/about/metadata)):

```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

## Example

The `feature_gate` macro allows the specification of a single feature:

```rust
use feature_gate::feature_gate;

#[feature_gate("test")]
struct FeatureGated;

#[test]
fn it_works() {
    let _ = FeatureGated {};
}
```

The `feature_gate_ex` macro allows the specification of a complex set of requirements:

```rust
use feature_gate::feature_gate_ex;

#[feature_gate_ex(any(test, feature = "test"))]
struct FeatureGated;

#[test]
fn it_works() {
    let _ = FeatureGated {};
}
```
