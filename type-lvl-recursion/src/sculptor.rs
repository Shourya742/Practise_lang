use frunk::{HCons, HNil, hlist::Plucker};

trait Sculptor<Target, Indices> {
    type Remainder;

    fn sculpt(self) -> (Target, Self::Remainder);
}

impl<Source> Sculptor<HNil, HNil> for Source {
    // Since our target is HNil, we just return the Source
    type Remainder = Source;

    fn sculpt(self) -> (HNil, Self::Remainder) {
        (HNil, self)
    }
}

impl<THead, TTail, SHead, STail, IndexHead, IndexTail>
    Sculptor<HCons<THead, TTail>, HCons<IndexHead, IndexTail>> for HCons<SHead, STail>
where
    HCons<SHead, STail>: Plucker<THead, IndexHead>,
    // THe remainder of plucking the Target head type (THead) out of the source HList
    // must have a Sculptor implementation that lets us turn it into the tail type
    // of the Target HList (TTail) using the tail of the current indices (IndexTail)
    <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder: Sculptor<TTail, IndexTail>,
{
    type Remainder = <<HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder as Sculptor<
        TTail,
        IndexTail,
    >>::Remainder;

    fn sculpt(self) -> (HCons<THead, TTail>, Self::Remainder) {
        let (p, r): (
            THead,
            <HCons<SHead, STail> as Plucker<THead, IndexHead>>::Remainder,
        ) = self.pluck();
        let (tail, tail_remainder): (TTail, Self::Remainder) = r.sculpt();
        (HCons { head: p, tail }, tail_remainder)
    }
}
