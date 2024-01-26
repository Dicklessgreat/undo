use super::Socket;
use crate::Record;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use heapless::Deque;

/// Builder for a [`Record`].
///
/// # Examples
/// ```
/// # use undo::{Add, Record};
/// # let mut target = String::new();
/// let mut record = Record::builder()
///     .limit(100)
///     .capacity(100)
///     .connect(|s| { dbg!(s); })
///     .build();
/// # record.edit(&mut target, Add('a'));
/// ```
#[derive(Debug)]
pub struct Builder<E, const N: usize, S = ()> {
    limit: NonZeroUsize,
    saved: bool,
    socket: Socket<S>,
    pd: PhantomData<E>,
}

impl<E, const N: usize, S> Builder<E, N, S> {
    /// Sets the capacity for the record.
    // pub fn capacity<const M: usize>(mut self) -> Builder<E, M, S> {
    //     self
    // }

    /// Sets the `limit` of the record.
    ///
    /// # Panics
    /// Panics if `limit` is `0`.
    pub fn limit(mut self, limit: usize) -> Builder<E, N, S> {
        self.limit = NonZeroUsize::new(limit).expect("limit can not be `0`");
        self
    }

    /// Sets if the target is initially in a saved state.
    /// By default the target is in a saved state.
    pub fn saved(mut self, saved: bool) -> Builder<E, N, S> {
        self.saved = saved;
        self
    }

    /// Connects the slot.
    pub fn connect(mut self, slot: S) -> Builder<E, N, S> {
        self.socket = Socket::new(slot);
        self
    }

    /// Builds the record.
    pub fn build(self) -> Record<E, N, S> {
        Record {
            limit: self.limit,
            index: 0,
            saved: self.saved.then_some(0),
            socket: self.socket,
            entries: Deque::with_capacity(N),
        }
    }
}

impl<E, const N: usize, S> Default for Builder<E, N, S> {
    fn default() -> Self {
        Builder {
            limit: NonZeroUsize::new(usize::MAX).unwrap(),
            saved: true,
            socket: Socket::default(),
            pd: PhantomData,
        }
    }
}
