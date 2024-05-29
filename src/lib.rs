#![doc = include_str!("../README.md")]
#![no_std]

use core::marker::PhantomData;

/// Maps any given lifetime (that is bounded by `'outer`) to a concrete type.
///
/// The `'outer` lifetime is a constraint on the input lifetime `'inner` of the
/// associated type `In`. Generally, the type returned by `In` must outlive the
/// lifetime `'outer`.
///
/// Implementations of `Liftor` can generically implement for all lifetimes
/// `'outer` that borrow the type returned by `In`. For example, `Ref<T>`
/// implements `Liftor<'outer>` for all lifetimes `'outer` where `T: 'outer`.
/// This allows `Ref<T>` and other `Liftor`s that use this pattern to work even
/// on non-`'static` types, i.e. types that borrow data with a lifetime that is
/// not `'static`.
///
/// # Example
///
/// ```rust
/// use liftor::Liftor;
///
/// #[derive(Debug, PartialEq)]
/// enum List<'a, T> {
///     Empty,
///     Cons(T, &'a List<'a, T>),
/// }
///
/// impl<'a, T> Liftor<'a> for List<'a, T> {
///     type In<'b> = List<'b, T> where 'a: 'b;
/// }
///
/// trait Factory<'outer, Item: Liftor<'outer>> {
///     fn acquire<R, Cb>(self, cb: Cb) -> R
///     where
///         for<'inner> Cb: FnOnce(Item::In<'inner>) -> R;
/// }
///
/// struct ExampleListFactory;
/// impl Factory<'static, List<'static, i32>> for ExampleListFactory {
///     fn acquire<R, Cb>(self, cb: Cb) -> R
///     where
///         for<'inner> Cb: FnOnce(List<'inner, i32>) -> R,
///     {
///         let list = List::Cons(1, &List::Cons(2, &List::Empty));
///         cb(list)
///     }
/// }
///
/// ExampleListFactory.acquire(|list| {
///     assert_eq!(list, List::Cons(1, &List::Cons(2, &List::Empty)));
/// });
/// ```
pub trait Liftor<'outer> {
    /// The type that the lifetime `'inner` is mapped to.
    ///
    /// The type need not use the lifetime `'inner` directly, but it must
    /// outlive the `'inner` lifetime.
    type In<'inner>
    where
        'outer: 'inner;
}

/// Maps any lifetime to the type `T`.
///
/// This is a simple `Liftor` implementation that ignores the lifetime parameter
/// and always returns the same type.
///
/// Consider a `Callback` trait that is generic over any `Liftor`:
///
/// ```rust
/// use liftor::{Liftor, Owned};
///
/// trait Callback<'data, Input: Liftor<'data>> {
///     fn call(&self, input: Input::In<'_>);
/// }
///
/// struct MyCallback;
/// impl Callback<'static, Owned<i32>> for MyCallback {
///     fn call(&self, input: i32) {
///         println!("Received: {}", input);
///     }
/// }
/// ```
///
/// This example shows how you could instantiate a `Callback` that accepts an
/// `i32` by value by parameterizing it with `Owned<i32>`. The `call` method
/// can then accept an `i32` by value.
pub struct Owned<T>(PhantomData<T>);
impl<'outer, T> Liftor<'outer> for Owned<T>
where
    T: 'outer,
{
    type In<'inner> = T where 'outer: 'inner;
}

/// Maps any lifetime to a reference to `T`.
///
/// Consider a `Callback` trait that is generic over any `Liftor`:
///
/// ```rust
/// use liftor::{Liftor, Ref};
///
/// trait Callback<'data, Input: Liftor<'data>> {
///    fn call(&self, input: Input::In<'_>);
/// }
///
/// struct MyCallback;
/// impl Callback<'static, Ref<i32>> for MyCallback {
///     fn call(&self, input: &i32) {
///         println!("Received: {}", input);
///     }
/// }
/// ```
///
/// This example shows how you could instantiate a `Callback` that accepts an
/// `i32` by reference by parameterizing it with `Ref<i32>`. The `call` method
/// can then accept an `&i32`.
pub struct Ref<T: ?Sized>(PhantomData<T>);
impl<'outer, T> Liftor<'outer> for Ref<T>
where
    T: 'outer + ?Sized,
{
    type In<'inner> = &'inner T where 'outer: 'inner;
}

/// Maps any lifetime to a mutable reference to `T`.
///
/// Consider a `Callback` trait that is generic over any `Liftor`:
///
/// ```rust
/// use liftor::{Liftor, RefMut};
///
/// trait Callback<'data, Input: Liftor<'data>> {
///    fn call(&self, input: Input::In<'_>);
/// }
///
/// struct MyCallback;
/// impl Callback<'static, RefMut<i32>> for MyCallback {
///     fn call(&self, input: &mut i32) {
///         println!("Received: {}", input);
///     }
/// }
/// ```
///
/// This example shows how you could instantiate a `Callback` that accepts an
/// `i32` by mutable reference by parameterizing it with `RefMut<i32>`. The
/// `call` method can then accept an `&mut i32`.
pub struct RefMut<T: ?Sized>(PhantomData<T>);
impl<'outer, T> Liftor<'outer> for RefMut<T>
where
    T: 'outer + ?Sized,
{
    type In<'inner> = &'inner mut T where 'outer: 'inner;
}

/// A type alias for the `In` associated type of a `Liftor`. This can be used to
/// avoid complicated type names like `<L as Liftor<'outer>>::In<'inner>`.
///
/// # Example
///
/// ```rust
/// use liftor::{In, Liftor, Owned, Ref};
///
/// fn example<'outer, L: Liftor<'outer>>(input: In<'outer, L>)
/// where
///     In<'outer, L>: std::fmt::Debug,
/// {
///     println!("{:?}", input);
/// }
///
/// example::<Owned<i32>>(42);
/// example::<Ref<i32>>(&42);
/// ```
pub type In<'inner, L> = <L as Liftor<'inner>>::In<'inner>;

mod tuples;

#[cfg(test)]
mod tests;
