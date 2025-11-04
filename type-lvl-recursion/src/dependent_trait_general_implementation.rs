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

/// Instead of just A, we introduce another type parameter, B, which
/// is passed as the type parameter for the Cup that we want to add with
impl<A, B> Add<Cup<B>> for Cup<A>
// This next line means "A must have an Add<B> implementation"
where
    A: Add<B>,
{
    // The Output associated type now depends on the Output of <A as Add<B>>
    type Output = Cup<<A as Add<B>>::Output>;

    fn add(self, rhs: Cup<B>) -> Self::Output {
        // Notice that we can use the operator "+"
        let added_content = self.content.add(rhs.content);
        Cup {
            content: added_content,
        }
    }
}
