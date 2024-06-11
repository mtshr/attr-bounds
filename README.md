attr-bounds
===========

An attribute macro to stipulate bounds.

The attribute applies bounds to `struct`s, `enum`s, `union`s, `trait`s, `fn`s, and `impl` blocks.

```rust
use attr_bounds::bounds;

#[bounds(T: Copy)]
pub struct Wrapper<T>(T);

let var = Wrapper(42);
```

## Usage notes

Basically, the attribute is designed to be used for conditional compilation and otherwise you will not need the attribute.

```rust
use attr_bounds::bounds;

#[cfg(feature = "unstable_feature_a")]
pub trait UnstableA {}
#[cfg(feature = "unstable_feature_b")]
pub trait UnstableB {}

#[cfg_attr(feature = "unstable_feature_a", bounds(Self: UnstableA))]
#[cfg_attr(feature = "unstable_feature_b", bounds(Self: UnstableB))]
pub trait Trait {}

#[cfg(feature = "unstable_feature_a")]
impl UnstableA for () {}
#[cfg(feature = "unstable_feature_b")]
impl UnstableB for () {}

impl Trait for () {}
```
