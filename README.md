A macro that expands to the default implementation of a trait on a collection of types.

# Example

```rust
trait Show: Display + Sized {
    fn show(&self) {
        println!("{self}");
    }
}

// Uses the default implementation of Show to implement it on all the provided types.
default_impl!(Show, u8, u16, u32, String, isize);
```

what `default_impl` expands to in this case:
```rust
impl Show for u8 {}
impl Show for u16 {}
impl Show for u32 {}
impl Show for String {}
impl Show for isize {}
```