/// The Add trait which exist in core::ops, copied verbatim here.
///
/// Note that the Add trait has a right hand side (RHS) type parameter
/// to represent the type that the implementing trait is being added
/// with
pub trait Add<RHS = Self> {
    /// The resulting type after applying the `+` operator
    type Output;

    /// The method for the `+` operator
    fn add(self, rhs: RHS) -> Self::Output;
}

/// Our Cup struct. We signal that its contents can be
/// anything because if has an unrestricted type parameter
/// of A
struct Cup<A> {
    content: A,
}

/// In our case, we want to implement Add<Cup<A>> because we want to add
/// 2 cups with the same content type together, but we don't know in advance
/// what kind of content would be in them; hence we keep
/// it parameterized with A.
///
/// Thus, we write an implementation of Cup for Add, but add a restriction:
/// the implementation only exists for Cups where the content is bound to a
/// type that is already implements the Add trait (thus "A: Add<A>")
impl<A: Add<A>> Add<Cup<A>> for Cup<A> {
    /// This is what is called an associated type
    /// Here, Output is the type that will be returned
    /// from the add operation
    type Output = Cup<<A as Add<A>>::Output>;

    fn add(self, rhs: Cup<A>) -> Self::Output {
        // Here, we make use of the Add trait for A to add
        // the contents from both cups together.
        let added_content = self.content.add(rhs.content);
        Cup {
            content: added_content,
        }
    }
}
