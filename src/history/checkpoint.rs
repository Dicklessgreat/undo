use crate::{At, Edit, History, Slot};
use heapless::Vec;

#[derive(Debug)]
enum CheckpointEntry {
    Edit(usize),
    Undo,
    Redo,
}

/// Wraps a [`History`] and gives it checkpoint functionality.
#[derive(Debug)]
pub struct Checkpoint<'a, E, const N: usize, const M: usize, S> {
    history: &'a mut History<E, N, S>,
    entries: Vec<CheckpointEntry, M>,
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
    /// Calls the [`History::edit`] method.
    pub fn edit(&mut self, target: &mut E::Target, edit: E) -> E::Output {
        if self
            .entries
            .push(CheckpointEntry::Edit(self.history.root))
            .is_err()
        {
            panic!("Entry limit exceeded!!")
        }
        self.history.edit(target, edit)
    }

    /// Calls the [`History::undo`] method.
    pub fn undo(&mut self, target: &mut E::Target) -> Option<E::Output> {
        if self.entries.push(CheckpointEntry::Undo).is_err() {
            panic!("Entry limit exceeded!!")
        }
        self.history.undo(target)
    }

    /// Calls the [`History::redo`] method.
    pub fn redo(&mut self, target: &mut E::Target) -> Option<E::Output> {
        if self.entries.push(CheckpointEntry::Redo).is_err() {
            panic!("Entry limit exceeded!!")
        }
        self.history.redo(target)
    }

    /// Cancels the changes and consumes the checkpoint.
    pub fn cancel(self, target: &mut E::Target) -> Vec<E::Output, M> {
        todo!("find rev() alternative!!");
        self.entries
            .into_iter()
            // .rev()
            .filter_map(|entry| match entry {
                CheckpointEntry::Edit(root) => {
                    let output = self.history.undo(target)?;
                    if self.history.root == root {
                        self.history.record.entries.pop_back();
                    } else {
                        // If a new root was created when we edited earlier,
                        // we remove it and append the entries to the previous root.
                        let mut branch = self.history.branches.remove(root);
                        debug_assert_eq!(branch.parent, self.history.head());

                        let new = At::new(root, self.history.record.head());
                        let (_, rm_saved) = self.history.record.rm_tail();
                        for en in branch.entries {
                            let _ = self.history.record.entries.push_back(en);
                        }
                        self.history.set_root(new, rm_saved);
                    }
                    Some(output)
                }
                CheckpointEntry::Undo => self.history.redo(target),
                CheckpointEntry::Redo => self.history.undo(target),
            })
            .collect()
    }
}

impl<'a, E, const N: usize, const M: usize, S> From<&'a mut History<E, N, S>>
    for Checkpoint<'a, E, N, M, S>
{
    fn from(history: &'a mut History<E, N, S>) -> Self {
        Checkpoint {
            history,
            entries: Vec::new(),
        }
    }
}
