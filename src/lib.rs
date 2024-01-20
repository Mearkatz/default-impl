/// Expands to the default implementation of a trait on all the types provided
/// # Example
/// ```rust
/// trait Show: Display + Sized {
///     fn show(&self) {
///         println!("{self}");
///     }
/// }
///
/// default_impl!(Show, u8, u16, u32, String, isize);
/// ```
#[macro_export]
macro_rules! default_impl {
    ($tr: ty,$($x: ty),*) => {
        $(
            impl $tr for $x {}
        )*
    };
}
