use super::*;
use static_assertions::assert_type_eq_all;

// Note: this crate uses static_assertions to check type equality, so "tests"
// that use `assert_type_eq_all!` are actually compile-time checks, not unit
// tests.

// Sanity tests for the provided `Liftor` implementations.
assert_type_eq_all!(<Owned<i32> as Liftor<'_>>::In<'_>, i32);
assert_type_eq_all!(<Ref<i32> as Liftor<'_>>::In<'_>, &i32);
assert_type_eq_all!(<RefMut<i32> as Liftor<'_>>::In<'_>, &mut i32);

// Ref and RefMut should work for unsized types.
assert_type_eq_all!(<Ref<str> as Liftor<'_>>::In<'_>, &str);
assert_type_eq_all!(<RefMut<str> as Liftor<'_>>::In<'_>, &mut str);
assert_type_eq_all!(<Ref<[i32]> as Liftor<'_>>::In<'_>, &[i32]);
assert_type_eq_all!(<RefMut<[i32]> as Liftor<'_>>::In<'_>, &mut [i32]);
trait Foo {}
assert_type_eq_all!(<Ref<dyn Foo> as Liftor<'_>>::In<'_>, &dyn Foo);
assert_type_eq_all!(<RefMut<dyn Foo> as Liftor<'_>>::In<'_>, &mut dyn Foo);
assert_type_eq_all!(<Ref<[i32; 3]> as Liftor<'_>>::In<'_>, &[i32; 3]);


// Tuples of `Liftor`s should be `Liftor`s of tuples.
assert_type_eq_all!(<() as Liftor<'_>>::In<'_>, ());
assert_type_eq_all!(
    <(Owned<i32>, Owned<i32>) as Liftor<'_>>::In<'_>,
    (i32, i32)
);
assert_type_eq_all!(<(Ref<i32>, Ref<i32>) as Liftor<'_>>::In<'_>, (&i32, &i32));
assert_type_eq_all!(
    <(RefMut<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (&mut i32, &mut i32)
);
assert_type_eq_all!(
    <(Owned<i32>, Ref<i32>) as Liftor<'_>>::In<'_>,
    (i32, &i32)
);
assert_type_eq_all!(
    <(Ref<i32>, Owned<i32>) as Liftor<'_>>::In<'_>,
    (&i32, i32)
);
assert_type_eq_all!(
    <(Owned<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (i32, &mut i32)
);
assert_type_eq_all!(
    <(RefMut<i32>, Ref<i32>, Owned<i32>) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, i32)
);
assert_type_eq_all!(
    <(Ref<i32>, RefMut<i32>, Owned<i32>) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, i32)
);
assert_type_eq_all!(
    <(Ref<i32>, Owned<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (&i32, i32, &mut i32)
);
assert_type_eq_all!(
    <(Owned<i32>, Ref<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (i32, &i32, &mut i32)
);
assert_type_eq_all!(
    <(RefMut<i32>, Ref<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    <(Ref<i32>, RefMut<i32>, Ref<i32>, RefMut<i32>) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    <(RefMut<i32>, Ref<i32>, RefMut<i32>, Ref<i32>) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    <(Ref<i32>, RefMut<i32>, Ref<i32>, RefMut<i32>, Owned<i32>) as Liftor<
        '_,
    >>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32, i32)
);
assert_type_eq_all!(
    <(RefMut<i32>, Ref<i32>, RefMut<i32>, Ref<i32>, Owned<i32>) as Liftor<
        '_,
    >>::In<'_>,
    (&mut i32, &i32, &mut i32, &i32, i32)
);
assert_type_eq_all!(
    <(
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>
    ) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    <(
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>
    ) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    <(
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Owned<i32>
    ) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, i32)
);
assert_type_eq_all!(
    <(
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        Owned<i32>
    ) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32, i32)
);
assert_type_eq_all!(
    <(
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>
    ) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    <(
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>
    ) as Liftor<'_>>::In<'_>,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    <(
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Ref<i32>,
        RefMut<i32>,
        Owned<i32>
    ) as Liftor<'_>>::In<'_>,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32, i32)
);
