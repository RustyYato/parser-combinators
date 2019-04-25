use super::*;

impl<Input, P: ParserMut<Input>> ParserCombinators<Input> for P {}

pub trait ParserCombinators<Input> {
    #[inline]
    fn map<F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        Map<Self, F>: ParserMut<Input>,
    {
        Map(self, f)
    }

    #[inline]
    fn map_err<F>(self, f: F) -> MapErr<Self, F>
    where
        Self: Sized,
        MapErr<Self, F>: ParserMut<Input>,
    {
        MapErr(self, f)
    }

    #[inline]
    fn map_both<F, G>(self, f: F, g: G) -> MapBoth<Self, F, G>
    where
        Self: Sized,
        MapBoth<Self, F, G>: ParserMut<Input>,
    {
        MapBoth(self, f, g)
    }

    #[inline]
    fn flat_map<F>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        FlatMap<Self, F>: ParserMut<Input>,
    {
        FlatMap(self, f)
    }

    #[inline]
    fn flat_map_err<F>(self, f: F) -> FlatMapErr<Self, F>
    where
        Self: Sized,
        FlatMapErr<Self, F>: ParserMut<Input>,
    {
        FlatMapErr(self, f)
    }

    #[inline]
    fn flat_map_both<F, G>(self, f: F, g: G) -> FlatMapBoth<Self, F, G>
    where
        Self: Sized,
        FlatMapBoth<Self, F, G>: ParserMut<Input>,
    {
        FlatMapBoth(self, f, g)
    }

    #[inline]
    fn then<P>(self, p: P) -> Then<Self, P>
    where
        Self: Sized,
        Then<Self, P>: ParserMut<Input>,
    {
        Then(self, p)
    }

    #[inline]
    fn or<P>(self, p: P) -> Or<Self, P>
    where
        Self: Sized,
        Or<Self, P>: ParserMut<Input>,
    {
        Or(self, p)
    }

    #[inline]
    fn and_then<F>(self, f: F) -> AndThen<Self, F>
    where
        Self: Sized,
        AndThen<Self, F>: ParserMut<Input>,
    {
        AndThen(self, f)
    }

    #[inline]
    fn or_else<F>(self, f: F) -> OrElse<Self, F>
    where
        Self: Sized,
        OrElse<Self, F>: ParserMut<Input>,
    {
        OrElse(self, f)
    }

    #[inline]
    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        Self: Sized,
        Inspect<Self, F>: ParserMut<Input>,
    {
        Inspect(self, f)
    }

    #[inline]
    fn inspect_input<F>(self, f: F) -> InspectInput<Self, F>
    where
        Self: Sized,
        InspectInput<Self, F>: ParserMut<Input>,
    {
        InspectInput(self, f)
    }

    #[inline]
    fn filter<F>(self, f: F) -> Filter<Self, F>
    where
        Self: Sized,
        Filter<Self, F>: ParserMut<Input>,
    {
        Filter(self, f)
    }

    #[inline]
    fn filter_input<F>(self, f: F) -> FilterInput<Self, F>
    where
        Self: Sized,
        FilterInput<Self, F>: ParserMut<Input>,
    {
        FilterInput(self, f)
    }

    #[inline]
    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
        Optional<Self>: ParserMut<Input>,
    {
        Optional(self)
    }

    #[inline]
    fn zero_or_more<F>(self, f: F) -> ZeroOrMore<Self, F>
    where
        Self: Sized,
        ZeroOrMore<Self, F>: ParserMut<Input>,
    {
        ZeroOrMore(self, f)
    }

    #[inline]
    fn one_or_more<F>(self, f: F) -> OneOrMore<Self, F>
    where
        Self: Sized,
        OneOrMore<Self, F>: ParserMut<Input>,
    {
        OneOrMore(ZeroOrMore(self, f))
    }

    #[inline]
    fn repeat<F, R>(self, r: R, f: F) -> Repeat<Self, F, R>
    where
        Self: Sized,
        Repeat<Self, F, R>: ParserMut<Input>,
    {
        Repeat(self, f, r)
    }
}
