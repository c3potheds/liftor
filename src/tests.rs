use static_assertions::assert_type_eq_all;

use super::*;

// Note: this crate uses static_assertions to check type equality, so "tests"
// that use `assert_type_eq_all!` are actually compile-time checks, not unit
// tests.

// Sanity tests for the provided `Liftor` implementations.
assert_type_eq_all!(In<'_, Owned<i32>>, i32);
assert_type_eq_all!(In<'_, Ref<i32>>, &i32);
assert_type_eq_all!(In<'_, RefMut<i32>>, &mut i32);

// Ref and RefMut should work for unsized types.
assert_type_eq_all!(In<'_, Ref<str>>, &str);
assert_type_eq_all!(In<'_, RefMut<str>>, &mut str);
assert_type_eq_all!(In<'_, Ref<[i32]>>, &[i32]);
assert_type_eq_all!(In<'_, RefMut<[i32]>>, &mut [i32]);

// Ref and RefMut should work for trait objects.
#[allow(unused)]
trait Foo {}

assert_type_eq_all!(In<'_, Ref<dyn Foo>>, &dyn Foo);
assert_type_eq_all!(In<'_, RefMut<dyn Foo>>, &mut dyn Foo);
assert_type_eq_all!(In<'_, Ref<str>>, &str);

assert_type_eq_all!(In<'_, Ref<dyn Foo>>, &dyn Foo);
assert_type_eq_all!(In<'_, RefMut<dyn Foo>>, &mut dyn Foo);

assert_type_eq_all!(In<'_, Ref<[i32; 3]>>, &[i32; 3]);
assert_type_eq_all!(In<'_, RefMut<[i32; 3]>>, &mut [i32; 3]);

// Tuples of `Liftor`s should be `Liftor`s of tuples.
assert_type_eq_all!(In<'_, ()>, ());
assert_type_eq_all!(In<'_, (Owned<i32>, Owned<i32>)>, (i32, i32));
assert_type_eq_all!(In<'_, (Ref<i32>, Ref<i32>)>, (&i32, &i32));
assert_type_eq_all!(In<'_, (RefMut<i32>, RefMut<i32>)>, (&mut i32, &mut i32));
assert_type_eq_all!(In<'_, (Owned<i32>, Ref<i32>)>, (i32, &i32));
assert_type_eq_all!(In<'_, (Ref<i32>, Owned<i32>)>, (&i32, i32));
assert_type_eq_all!(In<'_, (Owned<i32>, RefMut<i32>)>, (i32, &mut i32));
assert_type_eq_all!(
    In<'_, (RefMut<i32>, Ref<i32>, Owned<i32>)>,
    (&mut i32, &i32, i32)
);
assert_type_eq_all!(
    In<'_, (Ref<i32>, RefMut<i32>, Owned<i32>)>,
    (&i32, &mut i32, i32)
);
assert_type_eq_all!(
    In<'_, (Ref<i32>, Owned<i32>, RefMut<i32>)>,
    (&i32, i32, &mut i32)
);
assert_type_eq_all!(
    In<'_, (Owned<i32>, Ref<i32>, RefMut<i32>)>,
    (i32, &i32, &mut i32)
);
assert_type_eq_all!(
    In<'_, (RefMut<i32>, Ref<i32>, RefMut<i32>)>,
    (&mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    In<'_, (Ref<i32>, RefMut<i32>, Ref<i32>, RefMut<i32>)>,
    (&i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    In<'_, (RefMut<i32>, Ref<i32>, RefMut<i32>, Ref<i32>)>,
    (&mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    In<'_, (Ref<i32>, RefMut<i32>, Ref<i32>, RefMut<i32>, Owned<i32>)>,
    (&i32, &mut i32, &i32, &mut i32, i32)
);
assert_type_eq_all!(
    In<'_, (RefMut<i32>, Ref<i32>, RefMut<i32>, Ref<i32>, Owned<i32>)>,
    (&mut i32, &i32, &mut i32, &i32, i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>
        ),
    >,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>
        ),
    >,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Owned<i32>
        ),
    >,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            Owned<i32>
        ),
    >,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32, i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>
        ),
    >,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>
        ),
    >,
    (&mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32)
);
assert_type_eq_all!(
    In<
        '_,
        (
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Ref<i32>,
            RefMut<i32>,
            Owned<i32>
        ),
    >,
    (&i32, &mut i32, &i32, &mut i32, &i32, &mut i32, &i32, &mut i32, i32)
);
