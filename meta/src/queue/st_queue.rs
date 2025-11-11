/// A strongly typed non-empty queue of heterogeneous elements.
///
/// There exist two implementations:
/// * [`QueueSingle`] which includes exactly one element, and
/// * [`Queue`] containing multiple (>=2) elements.
///
/// Also see [`define_queue`] macro to define a queue of heterogeneous elements
/// all of which exhibit a common behavior, or implement a common set of traits.
pub trait StQueue {
    /// Type of the queue obtained by adding an element of type `Elem` to this queue.
    type PushBack<Elem>: StQueue;

    /// Type of the element at the front of the queue.
    type Front;

    /// Type of the queue that would be obtained by popping the `Front` element of the queue.
    type Back: StQueue;

    /// Number of elements in the queue.
    const LEN: usize;

    /// Pushes the `element` and returns the resulting queue.
    ///
    /// Type of the resulting queue is know by the generic associated type `Self::PushBack<Elem>`.
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem>;

    /// Pushes the `element` and returns the resulting queue.
    ///
    /// This method is provided for convention. Length of the queue is actually known by the constant `Self::LEN`.
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns a reference to the element in the front of the queue.
    fn front(&self) -> &Self::Front;

    /// Returns a mutable reference to the element in the front of the queue.
    fn front_mut(&mut self) -> &mut Self::Front;

    /// Consumes the queue and returns the element in the front of the queue.
    fn into_front(self) -> Self::Front;
}
