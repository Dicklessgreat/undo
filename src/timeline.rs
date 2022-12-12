use crate::entry::{Entries, Entry};
use crate::slot::{NoOp, Signal, Slot, SW};
use crate::{Action, Merged};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug)]
pub(crate) struct Timeline<E, S = NoOp> {
    pub entries: E,
    pub current: usize,
    pub saved: Option<usize>,
    pub slot: SW<S>,
}

impl<E: Entries, S> Timeline<E, S> {
    pub fn can_undo(&self) -> bool {
        self.current > 0
    }

    pub fn can_redo(&self) -> bool {
        self.current < self.entries.len()
    }

    pub fn is_saved(&self) -> bool {
        self.saved.map_or(false, |saved| saved == self.current)
    }
}

impl<E, S> Timeline<E, S>
where
    E: Entries,
    E::Item: Action,
    S: Slot,
{
    pub fn apply(
        &mut self,
        target: &mut <E::Item as Action>::Target,
        mut action: E::Item,
    ) -> (<E::Item as Action>::Output, bool, E) {
        let output = action.apply(target);
        // We store the state of the stack before adding the entry.
        let current = self.current;
        let could_undo = self.can_undo();
        let could_redo = self.can_redo();
        let was_saved = self.is_saved();
        // Pop off all elements after len from entries.
        let tail = self.entries.split_off(current);
        // Check if the saved state was popped off.
        self.saved = self.saved.filter(|&saved| saved <= current);
        // Try to merge actions unless the target is in a saved state.
        let merged = match self.entries.back_mut() {
            Some(last) if !was_saved => last.action.merge(action),
            _ => Merged::No(action),
        };
        let merged_or_annulled = match merged {
            Merged::Yes => true,
            Merged::Annul => {
                self.entries.pop_back();
                self.current -= 1;
                true
            }
            // If actions are not merged or annulled push it onto the storage.
            Merged::No(action) => {
                // If limit is reached, pop off the first action.
                if self.entries.limit() == self.current {
                    self.entries.pop_front();
                    self.saved = self.saved.and_then(|saved| saved.checked_sub(1));
                } else {
                    self.current += 1;
                }
                self.entries.push_back(Entry::from(action));
                false
            }
        };
        self.slot.emit_if(could_redo, Signal::Redo(false));
        self.slot.emit_if(!could_undo, Signal::Undo(true));
        self.slot.emit_if(was_saved, Signal::Saved(false));
        (output, merged_or_annulled, tail)
    }

    pub fn undo(
        &mut self,
        target: &mut <E::Item as Action>::Target,
    ) -> Option<<E::Item as Action>::Output> {
        self.can_undo().then(|| {
            let was_saved = self.is_saved();
            let old = self.current;
            let output = self.entries[self.current - 1].action.undo(target);
            self.current -= 1;
            let is_saved = self.is_saved();
            self.slot
                .emit_if(old == self.entries.len(), Signal::Redo(true));
            self.slot.emit_if(old == 1, Signal::Undo(false));
            self.slot
                .emit_if(was_saved != is_saved, Signal::Saved(is_saved));
            output
        })
    }

    pub fn redo(
        &mut self,
        target: &mut <E::Item as Action>::Target,
    ) -> Option<<E::Item as Action>::Output> {
        self.can_redo().then(|| {
            let was_saved = self.is_saved();
            let old = self.current;
            let output = self.entries[self.current].action.redo(target);
            self.current += 1;
            let is_saved = self.is_saved();
            self.slot
                .emit_if(old == self.entries.len() - 1, Signal::Redo(false));
            self.slot.emit_if(old == 0, Signal::Undo(true));
            self.slot
                .emit_if(was_saved != is_saved, Signal::Saved(is_saved));
            output
        })
    }

    pub fn set_saved(&mut self, saved: bool) {
        let was_saved = self.is_saved();
        if saved {
            self.saved = Some(self.current);
            self.slot.emit_if(!was_saved, Signal::Saved(true));
        } else {
            self.saved = None;
            self.slot.emit_if(was_saved, Signal::Saved(false));
        }
    }

    pub fn clear(&mut self) {
        let could_undo = self.can_undo();
        let could_redo = self.can_redo();
        self.entries.clear();
        self.saved = self.is_saved().then_some(0);
        self.current = 0;
        self.slot.emit_if(could_undo, Signal::Undo(false));
        self.slot.emit_if(could_redo, Signal::Redo(false));
    }
}

impl<E, S> Timeline<E, S>
where
    E: Entries,
    E::Item: Action<Output = ()>,
    S: Slot,
{
    pub fn go_to(
        &mut self,
        target: &mut <E::Item as Action>::Target,
        current: usize,
    ) -> Option<<E::Item as Action>::Output> {
        if current > self.entries.len() {
            return None;
        }
        let could_undo = self.can_undo();
        let could_redo = self.can_redo();
        let was_saved = self.is_saved();
        // Temporarily remove slot so they are not called each iteration.
        let slot = self.slot.disconnect();
        // Decide if we need to undo or redo to reach current.
        let undo_or_redo = if current > self.current {
            Timeline::redo
        } else {
            Timeline::undo
        };

        while self.current != current {
            undo_or_redo(self, target);
        }

        let can_undo = self.can_undo();
        let can_redo = self.can_redo();
        let is_saved = self.is_saved();
        self.slot.connect(slot);
        self.slot
            .emit_if(could_undo != can_undo, Signal::Undo(can_undo));
        self.slot
            .emit_if(could_redo != can_redo, Signal::Redo(can_redo));
        self.slot
            .emit_if(was_saved != is_saved, Signal::Saved(is_saved));
        Some(())
    }

    pub fn revert(
        &mut self,
        target: &mut <E::Item as Action>::Target,
    ) -> Option<<E::Item as Action>::Output> {
        self.saved.and_then(|saved| self.go_to(target, saved))
    }
}
