use crate::{Edit, Record, Slot};
use heapless::Vec;

#[derive(Debug)]
enum QueueEntry<E> {
    Edit(E),
    Undo,
    Redo,
}

/// Wraps a [`Record`] and gives it batch queue functionality.
///
/// # Examples
/// ```
/// # use undo::{Add, Record};
/// let mut string = String::new();
/// let mut record = Record::new();
/// let mut queue = record.queue();
///
/// queue.edit(Add('a'));
/// queue.edit(Add('b'));
/// queue.edit(Add('c'));
/// assert_eq!(string, "");
///
/// queue.commit(&mut string);
/// assert_eq!(string, "abc");
/// ```
#[derive(Debug)]
pub struct Queue<'a, E, const N: usize, const M: usize, S> {
    record: &'a mut Record<E, N, S>,
    entries: Vec<QueueEntry<E>, M>,
}

impl<E, const N: usize, const M: usize, S> Queue<'_, E, N, M, S> {
    /// Reserves capacity for at least `additional` more entries in the queue.
    ///
    /// # Panics
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    // pub fn reserve(&mut self, additional: usize) {
    //     self.entries.reserve(additional);
    // }

    /// Queues a [`Record::edit`] call.
    pub fn edit(&mut self, edit: E) {
        if self.entries.push(QueueEntry::Edit(edit)).is_err() {
            panic!("Entry limit exceeded!!")
        }
    }

    /// Queues a [`Record::undo`] call.
    pub fn undo(&mut self) {
        if self.entries.push(QueueEntry::Undo).is_err() {
            panic!("Entry limit exceeded!!")
        }
    }

    /// Queues a [`Record::redo`] call.
    pub fn redo(&mut self) {
        if self.entries.push(QueueEntry::Redo).is_err() {
            panic!("Entry limit exceeded!!")
        }
    }

    /// Cancels the queued edits.
    pub fn cancel(self) {}
}

impl<E: Edit, const N: usize, const M: usize, S: Slot> Queue<'_, E, N, M, S> {
    /// Applies the queued edits.
    pub fn commit(self, target: &mut E::Target) -> Vec<E::Output, M> {
        self.entries
            .into_iter()
            .filter_map(|entry| match entry {
                QueueEntry::Edit(edit) => Some(self.record.edit(target, edit)),
                QueueEntry::Undo => self.record.undo(target),
                QueueEntry::Redo => self.record.redo(target),
            })
            .collect()
    }
}

impl<'a, E, const N: usize, const M: usize, S> From<&'a mut Record<E, N, S>>
    for Queue<'a, E, N, M, S>
{
    fn from(record: &'a mut Record<E, N, S>) -> Self {
        Queue {
            record,
            entries: Vec::new(),
        }
    }
}
