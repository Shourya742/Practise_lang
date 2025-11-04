use std::marker::PhantomData;

use frunk::HCons;

/// The new and improved Plucker trait
trait Plucker<Target, Index> {
    type Remainder;

    /// Pluck should return the target type and the Remainder in a pair.
    fn pluck(self) -> (Target, Self::Remainder);
}

/// This will be the type we'll use to denote that the Target is in the HEAD
enum Here {}

impl<Target, Tail> Plucker<Target, Here> for HCons<Target, Tail> {
    // Target is the head element, so the Remainder Type is the tail!
    type Remainder = Tail;

    fn pluck(self) -> (Target, Self::Remainder) {
        (self.head, self.tail)
    }
}

// Type for representing a not here Index
struct There<T>(PhantomData<T>);

// impl <Target, Head, Tail> Plucker<Target> for HCons<Head, Tail> where Tail: Plucker<Target> {
//     type Remainder = HCons<Head, <Tail as Plucker<Target>>::Remainder>;

//     fn pluck(self) -> (Target, Self::Remainder) {
//         let (tail_target, tail_remainder): (Target, <Tail as Plucker<Target>>::Remainder) = self.tail.pluck();
//         (
//             tail_target,
//             HCons { head: self.head, tail: tail_remainder }
//         )
//     }
// }

impl<Head, Tail, Target, TailIndex> Plucker<Target, There<TailIndex>> for HCons<Head, Tail>
// This where clause can be interpreted as "the target must be pluckable from the tail"
where
    Tail: Plucker<Target, TailIndex>,
{
    type Remainder = HCons<Head, <Tail as Plucker<Target, TailIndex>>::Remainder>;

    fn pluck(self) -> (Target, Self::Remainder) {
        let (target, tail_remainder): (Target, <Tail as Plucker<Target, TailIndex>>::Remainder) =
            <Tail as Plucker<Target, TailIndex>>::pluck(self.tail);
        (
            target,
            HCons {
                head: self.head,
                tail: tail_remainder,
            },
        )
    }
}
