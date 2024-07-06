use crate::Liftor;

/// The unit type is a `Liftor` that maps any lifetime to the unit type.
impl<'outer> Liftor<'outer> for () {
    type In<'inner> = () where 'outer: 'inner;
}

/// A tuple of [`Liftor`]s is a `Liftor` of tuples.
///
/// This lets you use tuples of `Liftor`s as `Liftor`s themselves. For example,
/// you can use a tuple of `Ref`s as a `Liftor` of a tuple of references, where
/// each reference in the tuple has the lifetime of the lifetime input to the
/// `In` associated type.
///
/// Tuples are supported for up to 16 elements.
impl<'outer, A, B> Liftor<'outer> for (A, B)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
{
    type In<'inner> = (A::In<'inner>, B::In<'inner>) where 'outer: 'inner;
}

impl<'outer, A, B, C> Liftor<'outer> for (A, B, C)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D> Liftor<'outer> for (A, B, C, D)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E> Liftor<'outer> for (A, B, C, D, E)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F> Liftor<'outer> for (A, B, C, D, E, F)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G> Liftor<'outer> for (A, B, C, D, E, F, G)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H> Liftor<'outer> for (A, B, C, D, E, F, G, H)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K, L> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
    L: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
        L::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K, L, M> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
    L: Liftor<'outer>,
    M: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
        L::In<'inner>,
        M::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K, L, M, N> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
    L: Liftor<'outer>,
    M: Liftor<'outer>,
    N: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
        L::In<'inner>,
        M::In<'inner>,
        N::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
    L: Liftor<'outer>,
    M: Liftor<'outer>,
    N: Liftor<'outer>,
    O: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
        L::In<'inner>,
        M::In<'inner>,
        N::In<'inner>,
        O::In<'inner>,
    )
    where
        'outer: 'inner;
}

impl<'outer, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> Liftor<'outer>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    A: Liftor<'outer>,
    B: Liftor<'outer>,
    C: Liftor<'outer>,
    D: Liftor<'outer>,
    E: Liftor<'outer>,
    F: Liftor<'outer>,
    G: Liftor<'outer>,
    H: Liftor<'outer>,
    I: Liftor<'outer>,
    J: Liftor<'outer>,
    K: Liftor<'outer>,
    L: Liftor<'outer>,
    M: Liftor<'outer>,
    N: Liftor<'outer>,
    O: Liftor<'outer>,
    P: Liftor<'outer>,
{
    type In<'inner> = (
        A::In<'inner>,
        B::In<'inner>,
        C::In<'inner>,
        D::In<'inner>,
        E::In<'inner>,
        F::In<'inner>,
        G::In<'inner>,
        H::In<'inner>,
        I::In<'inner>,
        J::In<'inner>,
        K::In<'inner>,
        L::In<'inner>,
        M::In<'inner>,
        N::In<'inner>,
        O::In<'inner>,
        P::In<'inner>,
    )
    where
        'outer: 'inner;
}
