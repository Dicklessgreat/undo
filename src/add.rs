use core::fmt::{self, Display, Formatter};
use heapless::String;

/// This is the edit used in all the examples.
///
/// Not part of the API and can change at any time.
#[doc(hidden)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Add<const SIZE: usize>(pub char);

impl<const SIZE: usize> crate::Edit for Add<SIZE> {
    type Target = String<SIZE>;
    type Output = ();

    fn edit(&mut self, string: &mut String<SIZE>) {
        let _ = string.push(self.0);
    }

    fn undo(&mut self, string: &mut String<SIZE>) {
        self.0 = string.pop().unwrap();
    }
}

impl<const SIZE: usize> Display for Add<SIZE> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Add '{}'", self.0)
    }
}
