use crate::{Edit, Entry, Record, Slot};
use heapless::Deque;
use heapless::Vec;

#[derive(Debug)]
enum CheckpointEntry<E, const N: usize> {
    Edit {
        saved: Option<usize>,
        tail: Deque<Entry<E>, N>,
    },
    Undo,
    Redo,
}

/// Wraps a [`Record`] and gives it checkpoint functionality.
#[derive(Debug)]
pub struct Checkpoint<'a, E, const N: usize, const M: usize, S> {
    record: &'a mut Record<E, N, S>,
    entries: Vec<CheckpointEntry<E, N>, M>,
}

impl<E, const N: usize, const M: usize, S> Checkpoint<'_, E, N, M, S> {
    /// Reserves capacity for at least `additional` more entries in the checkpoint.
    ///
    /// # Panics
    /// Panics if the new capacity exceeds `isize::MAX` bytes.
    // pub fn reserve(&mut self, additional: usize) {
    //     self.entries.reserve(additional);
    // }

    /// Commits the changes and consumes the checkpoint.
    pub fn commit(self) {}
}

impl<E: Edit, const N: usize, const M: usize, S: Slot> Checkpoint<'_, E, N, M, S> {
    /// Calls the `apply` method.
    pub fn edit(&mut self, target: &mut E::Target, edit: E) -> E::Output {
        let (output, _, tail, saved) = self.record.edit_and_push(target, Entry::new(edit));
        if self
            .entries
            .push(CheckpointEntry::Edit { saved, tail })
            .is_err()
        {
            panic!("Entry limit exceeded!!")
        }
        output
    }

    /// Calls the `undo` method.
    pub fn undo(&mut self, target: &mut E::Target) -> Option<E::Output> {
        let output = self.record.undo(target)?;
        if self.entries.push(CheckpointEntry::Undo).is_err() {
            panic!("Entry limit exceeded!!")
        }
        Some(output)
    }

    /// Calls the `redo` method.
    pub fn redo(&mut self, target: &mut E::Target) -> Option<E::Output> {
        let output = self.record.redo(target)?;
        if self.entries.push(CheckpointEntry::Redo).is_err() {
            panic!("Entry limit exceeded!!")
        }
        Some(output)
    }

    /// Cancels the changes and consumes the checkpoint.
    pub fn cancel(mut self, target: &mut E::Target) -> Vec<E::Output, M> {
        self.entries.as_mut_slice().reverse();
        self.entries
            .into_iter()
            .filter_map(|entry| match entry {
                CheckpointEntry::Edit { saved, tail } => {
                    let output = self.record.undo(target)?;
                    self.record.entries.pop_back();
                    for en in tail {
                        let _ = self.record.entries.push_front(en);
                    }
                    self.record.saved = self.record.saved.or(saved);
                    Some(output)
                }
                CheckpointEntry::Undo => self.record.redo(target),
                CheckpointEntry::Redo => self.record.undo(target),
            })
            .collect()
    }
}

impl<'a, E, const N: usize, const M: usize, S> From<&'a mut Record<E, N, S>>
    for Checkpoint<'a, E, N, M, S>
{
    fn from(record: &'a mut Record<E, N, S>) -> Self {
        Checkpoint {
            record,
            entries: Vec::new(),
        }
    }
}
