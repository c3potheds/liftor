A [`Liftor`] maps a given lifetime to a concrete type.

The name "liftor" is derived from **lifetime** and **functor**. Just as a
functor is a mapping of types to types, a liftor is a mapping of lifetimes
to types. It can also be thought of as "lifting" a concrete type to a set
of types parameterized by any lifetime.

This crate provides several implementations of `Liftor` for common cases:

  * [`Owned<T>`] maps any lifetime `'a` to `T` (ignoring the lifetime).
  * [`Ref<T>`] maps any lifetime `'a` to `&'a T`.
  * [`RefMut<T>`] maps any lifetime `'a` to `&'a mut T`.

You can also implement `Liftor` for your own types. For example, you may
have a struct that borrows strings. You can implement `Liftor` for this
struct to allow re-parameterizing the lifetimes of the strings that it
borrows:

```rust
use liftor::Liftor;

struct Contact<'a> {
    name: &'a str,
    phone: &'a str,
}

impl<'a> Liftor<'a> for Contact<'a> {
    // Remap the lifetime.
    type In<'b> = Contact<'b> where 'a: 'b;
}
```

The intended use of this crate is to decouple generic associated types with
lifetime parameters from some other limitations or unergonomic properties
of associated types. It is most useful in higher-order functions or traits
that need to be generic over reference and non-reference types, or In
continuation-passing style (CPS) where the lifetime of the data being passed
to a callback is not known at the time of definition.


# Motivation

Generic associated types are a powerful feature that allows a trait to
express a mapping of lifetimes to types. However, there are some subtle
limitations that make it difficult or impossible to express some things.
For example:

  * A trait can only have one implementation for a given type.
  * Bounds on function parameters

Consider a trait that uses a generic associated type (GAT) to implement the
"Factory" pattern, where the output of the factory may borrow data owned by
the factory itself. For example:

```rust
trait Factory {
    type Item<'a> where Self: 'a;
    fn create(&self) -> Self::Item<'_>;
}
```

This trait seems to abstract cleanly over the Factory pattern. Because of
the generic associated type mapping a lifetime to a concrete type, different
implementations of `Factory` can express whether they borrow data from the
factory, or even have a factory implementation return a reference to a
shared instance if desired.

```rust
# trait Factory {
#     type Item<'a> where Self: 'a;
#     fn create(&self) -> Self::Item<'_>;
# }
#
struct HelloWorld;
impl Factory for HelloWorld {
    type Item<'a> = &'a str;
    fn create(&self) -> &str {
        "Hello, world!"
    }
}

struct HelloName {
    name: String,
}
impl Factory for HelloName {
    type Item<'a> = String;
    fn create(&self) -> String {
        let Self { name } = self;
        format!("Hello, {name}!")
    }
}

struct HelloPrecomputed(String);
impl Factory for HelloPrecomputed {
    type Item<'a> = &'a str;
    fn create(&self) -> &str {
        &self.0
    }
}
```

However, the choice of a generic associated type locks us into a single
implementation of `Factory` for any given type. Suppose we want to have a
`Context` object that implements a `Factory` for `Foo` and a `Factory` for
`Bar`. Because `Item` is an associated type, generic or not, we cannot
implement `Factory` for `Foo` and `Bar` separately for the same `Context`.

```compile_fail
# trait Factory {
#     type Item<'a> where Self: 'a;
#     fn create(&self) -> Self::Item<'_>;
# }
#
# struct Foo;
# struct Bar;
#
struct Context;
impl Factory for Context {
    type Item<'a> = Foo;
    fn create(&self) -> Foo {
        Foo
    }
}
// ERROR: conflicting implementations of trait `Factory` for type `Context`
impl Factory for Context {
    type Item<'a> = Bar;
    fn create(&self) -> Bar {
        Bar
    }
}
```

It's also not possible to declare bounds on a function parameter that say
that a type implements `Factory` for both `Foo` and `Bar`:

```compile_fail
# trait Factory {
#     type Item<'a> where Self: 'a;
#     fn create(&self) -> Self::Item<'_>;
# }
#
# struct Foo;
# struct Bar;
#
fn use_foo_and_bar<C>(context: C)
where
    for <'a> C: Factory<Item<'a> = Foo> + Factory<Item<'a> = Bar>,
{
    // ERROR: type annotations needed; cannot satisfy
    // `<C as factory>::Item<'_> == Foo`
    let foo: Foo = context.create();
    let bar: Bar = context.create();
}
```

However, with a non-generic associated type that has is own generic
associated type, you can work around these limitations:

```rust
use liftor::Liftor;
use liftor::Owned;
use liftor::Ref;

trait Factory<'a> {
    type Item: Liftor<'a>;  
    fn create(&self) -> <Self::Item as Liftor<'a>>::In<'_>;
}

struct HelloWorld;
impl<'a> Factory<'a> for HelloWorld {
    type Item = Ref<str>;
    fn create(&self) -> &str {
        "Hello, world!"
    }
}

struct HelloName {
    name: String,
}
impl<'a> Factory<'a> for HelloName {
    type Item = Owned<String>;
    fn create(&self) -> String {
        let Self { name } = self;
        format!("Hello, {name}!")
    }
}

struct HelloPrecomputed(String);
impl<'a> Factory<'a> for HelloPrecomputed {
    type Item = Ref<str>;
    fn create(&self) -> &str {
        &self.0
    }
}

let hello_world = HelloWorld;
let hello_name = HelloName { name: String::from("Alice") };
let hello_precomputed = HelloPrecomputed("Hello, Bob!".to_owned());

assert_eq!(hello_world.create(), "Hello, world!");
assert_eq!(hello_name.create(), "Hello, Alice!");
assert_eq!(hello_precomputed.create(), "Hello, Bob!");
```

# Example 1: Factory with a callback

You can also create your own implementations of `Liftor`. For example, you
may have a struct that borrows strings. You can implement `Liftor` for this
struct to allow re-parameterizing the lifetimes of the strings that it
borrows:

```rust
use liftor::Liftor;

struct Contact<'a> {
    name: &'a str,
    phone: &'a str,
}

impl<'a> Liftor<'a> for Contact<'a> {
    // Remap the lifetime.
    type In<'b> = Contact<'b> where 'a: 'b;
}

// An abstraction over a factory that provides an item to a callback. The
// item is characterized by a Liftor rather than a concrete type so that the
// item doesn't have to necessarily outlive the Factory. This allows the
// item that is passed to the callback to borrow locally-owned data.
//
// Implementations can use Owned<T> as the Item parameter to pass an item
// with a 'static lifetime by value, or Ref<T> to pass an item by reference.
trait Factory<'outer, Item: Liftor<'outer>> {
    fn acquire<R, Cb>(self, cb: Cb) -> R
    where
        for<'inner> Cb: FnOnce(Item::In<'inner>) -> R;
}

struct ContactFactory;
impl<'a> Factory<'a, Contact<'a>> for ContactFactory {
   fn acquire<R, Cb>(self, cb: Cb) -> R
   where
       // Note: lifetime for Contact is elided, but thanks to the Liftor and
       // its use in the Factory trait, the lifetime of the Contact sent to
       // the callback is not necessarily 'a, and can be a lifetime that is
       // local to the scope of `acquire`.
       Cb: FnOnce(Contact) -> R,
   {
       // Locally-owned strings.
       let name = String::from("John Doe");
       let phone = String::from("555-5555");
       // Construct a Contact that borrows the locally-owned strings.
       let contact = Contact {
           name: &name,
           phone: &phone,
       };
       // "Returns" the Contact by passing it to the callback. If we had
       // tried to return it normally without callback-passing style, the
       // strings would have been dropped and the Contact would have been
       // left with dangling references, which would have caused a
       // borrow-checker error.
       cb(contact)
   }
}

ContactFactory.acquire(|contact| {
    assert_eq!(contact.name, "John Doe");
    assert_eq!(contact.phone, "555-5555");
});
```

In this example, the `Factory` trait allows constructing a given type
through **continuation-passing style** (**CPS**). The `acquire` method
takes a callback that accepts a `Contact` of any lifetime, which allows the
factory to pass a `Contact` that borrows locally-owned strings. In ad-hoc
CPS, one could use HRTBs, as seen in the `ContactFactory` implementation,
but it would not be possible to abstract over a `Factory` trait that allows
other ways of borrowing data, such as passing a reference or a mutable
reference to a `Contact`, without using `Liftor`.

# Example 2: A singly-linked list

As another example, consider a linked list implementation that uses a
reference to the next node in the list:

```rust
use liftor::Liftor;

#[derive(Debug, PartialEq)]
enum List<'a, T> {
    Empty,
    Cons(T, &'a List<'a, T>),
}
use List::*;

impl<'a, T> Liftor<'a> for List<'a, T> {
    type In<'b> = List<'b, T> where 'a: 'b;
}

trait Factory<'outer, Item: Liftor<'outer>> {
    fn acquire<R, Cb>(self, cb: Cb) -> R
    where
        for<'inner> Cb: FnOnce(Item::In<'inner>) -> R;
}

struct ExampleListFactory;
impl Factory<'static, List<'static, i32>> for ExampleListFactory {
    fn acquire<R, Cb>(self, cb: Cb) -> R
    where
        for<'inner> Cb: FnOnce(List<'inner, i32>) -> R,
    {
        let list = Cons(1, &Cons(2, &Empty));
        cb(list)
    }
}
struct StringListFactory;
// A List of borrowed data is parameterized by the lifetime of the data and
// the lifetime of reference to the next node in the list. Because the
// Liftor implementation for List only deals with the latter lifetime, we
// define a new custom liftor type that maps a given lifetime to a List that
// uses that lifetime for both the data and the reference to the next node.
struct StringList;
impl<'a> Liftor<'a> for StringList {
    type In<'b> = List<'b, &'b str> where 'a: 'b;
}
impl<'a> Factory<'static, StringList> for StringListFactory {
    fn acquire<R, Cb>(self, cb: Cb) -> R
    where
        for<'inner> Cb: FnOnce(List<'inner, &'inner str>) -> R,
    {
        let a = String::from("a");
        let b = String::from("b");
        let c = String::from("c");
        cb(Cons(a.as_str(), &Cons(b.as_str(), &Cons(c.as_str(), &Empty))))
    }
}

ExampleListFactory.acquire(|list| {
    assert_eq!(list, Cons(1, &Cons(2, &Empty)));
});
StringListFactory.acquire(|list| {
    assert_eq!(list, Cons("a", &Cons("b", &Cons("c", &Empty))));
});
```

Here we show where liftors truly shine. The `List` type is a recursive type
that borrows the next node in the list. As such, it is not possible to
return a `List` that borrows locally-owned data, but we can use continuation
passing style to pass the `List` to a callback. The `Factory` trait allows
expressing a generic way to acquire a type that borrows data regardless of
the lifetime of the data being borrowed, and the `Liftor` trait expresses
how the concrete type of the acquired type is parameterized by the lifetime.

Implementations of `Liftor` are most useful in higher-order functions to
parameterize lifetimes of generic arguments, particularly in CPS, where the
lifetime of the data being borrowed is not known at the time of definition.
