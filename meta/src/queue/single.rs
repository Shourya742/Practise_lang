use crate::queue::{multi::Queue, st_queue::StQueue};

/// A statically-typed queue containing exactly one element of type `Front`.
///
/// See also the other [`StQueue`] implementation [`Queue`] which can be
/// created by pushing a second element to this queue.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QueueSingle<Front> {
    pub(super) front: Front,
}

impl<F> StQueue for QueueSingle<F> {
    type PushBack<Elem> = Queue<F, QueueSingle<F>>;

    type Front = F;

    type Back = Self;

    const LEN: usize = 1;

    #[inline(always)]
    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem> {
        todo!()
    }

    #[inline(always)]
    fn front(&self) -> &Self::Front {
        &self.front
    }

    #[inline(always)]
    fn front_mut(&mut self) -> &mut Self::Front {
        &mut self.front
    }

    #[inline(always)]
    fn into_front(self) -> Self::Front {
        self.front
    }
}

impl<F> QueueSingle<F> {
    /// Creates a new statically-typed queue [`StQueue`] containing exactly one `element`.
    ///
    /// Alternatively, we can use multiple element queue's [`new`]. This is for convenience to
    /// allows to work with a single queue type while coding.
    pub fn new(element: F) -> Self {
        Self { front: element }
    }

    /// Pops and returns the element in the front of this queue.
    ///
    /// Since this element contains only one element, there is no remaining queue once the
    /// front is popped. Therefore, the return type is only the element rather than a tuple.
    pub fn pop(self) -> F {
        self.front
    }
}
