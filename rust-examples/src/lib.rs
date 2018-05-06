#![feature(generic_associated_types)]

trait Gatherable {
    type F<A>;
    type G<X>;
    fn gather<A>(x: Self::F<Self::G<A>>) -> Self::G<Self::F<A>>;
}
