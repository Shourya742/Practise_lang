use crate::queue::{single::QueueSingle, st_queue::StQueue};

pub struct Queue<Front, Back>
where
    Back: StQueue,
{
    f: Front,
    b: Back,
}

impl<F, B> StQueue for Queue<F, B>
where
    B: StQueue,
{
    type PushBack<Elem> = Queue<F, B::PushBack<Elem>>;

    type Front = F;

    type Back = B;

    const LEN: usize = 1 + B::LEN;

    #[inline(always)]
    fn front(&self) -> &Self::Front {
        &self.f
    }

    #[inline(always)]
    fn front_mut(&mut self) -> &mut Self::Front {
        &mut self.f
    }

    #[inline(always)]
    fn into_front(self) -> Self::Front {
        self.f
    }

    fn push<Elem>(self, element: Elem) -> Self::PushBack<Elem> {
        todo!()
    }
}

impl<F> Queue<F, QueueSingle<F>> {
    /// Creates a [`QueueSingle`] with exactly one `element`.
    ///
    /// Note that `Queue::new` is equivalent to `QueueSingle::new`. It is introduced for
    /// convenience allowing us to work only with the multiple element queue type `Queue`.
    pub fn new(element: F) -> QueueSingle<F> {
        QueueSingle::new(element)
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    pub fn from_fb(front: F, back: B) -> Self {
        Self { f: front, b: back }
    }
}

impl<F, B> Queue<F, B>
where
    B: StQueue,
{
    pub fn back(&self) -> &B {
        &self.b
    }

    pub fn back_mut(&mut self) -> &mut B {
        &mut self.b
    }

    pub fn front_back_mut(&mut self) -> (&mut F, &mut B) {
        (&mut self.f, &mut self.b)
    }

    pub fn into_back(self) -> B {
        self.b
    }

    pub fn pop(self) -> (F, B) {
        (self.f, self.b)
    }
}
