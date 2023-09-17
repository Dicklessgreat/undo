use crate::{At, Entry, Format, Record};
use core::fmt::{self, Write};
#[cfg(feature = "std")]
use std::time::SystemTime;

/// Configurable display formatting for the [`Record`].
pub struct Display<'a, E, S> {
    record: &'a Record<E, S>,
    format: Format,
}

impl<E, S> Display<'_, E, S> {
    /// Show colored output (on by default).
    ///
    /// Requires the `colored` feature to be enabled.
    #[cfg(feature = "colored")]
    pub fn colored(&mut self, on: bool) -> &mut Self {
        self.format.colored = on;
        self
    }

    /// Show detailed output (on by default).
    pub fn detailed(&mut self, on: bool) -> &mut Self {
        self.format.detailed = on;
        self
    }

    /// Show the current position in the output (on by default).
    pub fn head(&mut self, on: bool) -> &mut Self {
        self.format.head = on;
        self
    }

    /// Show the saved edit (on by default).
    pub fn saved(&mut self, on: bool) -> &mut Self {
        self.format.saved = on;
        self
    }
}

impl<E: fmt::Display, S> Display<'_, E, S> {
    fn fmt_list(
        &self,
        f: &mut fmt::Formatter,
        current: usize,
        entry: Option<&Entry<E>>,
        #[cfg(feature = "std")] now: SystemTime,
    ) -> fmt::Result {
        let at = At::root(current);
        self.format.position(f, at, false)?;

        #[cfg(feature = "std")]
        if let Some(entry) = entry {
            if self.format.detailed {
                self.format.elapsed(f, now, entry.created_at)?;
                self.format.text(f, ",", 3)?;
                self.format.elapsed(f, now, entry.updated_at)?;
            }
        }

        self.format.labels(
            f,
            at,
            At::root(self.record.index()),
            self.record.saved.map(At::root),
        )?;

        if let Some(entry) = entry {
            if self.format.detailed {
                writeln!(f)?;
                self.format.message(f, entry, None)?;
            } else {
                f.write_char(' ')?;
                self.format.message(f, entry, None)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl<'a, E, S> From<&'a Record<E, S>> for Display<'a, E, S> {
    fn from(record: &'a Record<E, S>) -> Self {
        Display {
            record,
            format: Format::default(),
        }
    }
}

impl<E: fmt::Display, S> fmt::Display for Display<'_, E, S> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(feature = "std")]
        let now = SystemTime::now();
        for (i, entry) in self.record.entries.iter().enumerate().rev() {
            self.fmt_list(
                f,
                i + 1,
                Some(entry),
                #[cfg(feature = "std")]
                now,
            )?;
        }
        self.fmt_list(
            f,
            0,
            None,
            #[cfg(feature = "std")]
            now,
        )
    }
}
