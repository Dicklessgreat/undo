#![cfg_attr(not(feature = "colored"), allow(unused_variables))]

use crate::At;
use core::fmt::Display;
use heapless::String;

#[cfg(feature = "colored")]
use colored::{Color, Colorize};
use core::fmt::{self, Write};
#[cfg(feature = "std")]
use std::time::SystemTime;

#[cfg(feature = "std")]
pub(crate) fn default_st_fmt(now: SystemTime, at: SystemTime) -> String {
    let elapsed = now.duration_since(at).unwrap_or_else(|e| e.duration());
    format!("{elapsed:.1?}")
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct Format {
    #[cfg(feature = "colored")]
    pub colored: bool,
    pub detailed: bool,
    pub head: bool,
    pub saved: bool,
}

impl Default for Format {
    fn default() -> Self {
        Format {
            #[cfg(feature = "colored")]
            colored: true,
            detailed: true,
            head: true,
            saved: true,
        }
    }
}

impl Format {
    pub fn level_text(self, f: &mut fmt::Formatter, text: &str, level: usize) -> fmt::Result {
        #[cfg(feature = "colored")]
        if self.colored {
            return write!(f, "{}", text.color(color_of_level(level)));
        }
        f.write_str(text)
    }

    pub fn message<const SIZE: usize>(
        self,
        f: &mut fmt::Formatter,
        msg: &impl Display,
        level: Option<usize>,
    ) -> fmt::Result {
        let mut buffer = String::<SIZE>::new();
        let _ = buffer.write_fmt(format_args!("{}", msg));
        let lines = buffer.lines();
        if self.detailed {
            for line in lines {
                if let Some(level) = level {
                    for i in 0..=level {
                        self.edge(f, i)?;
                        f.write_char(' ')?;
                    }
                }
                writeln!(f, "{}", line.trim())?;
            }
        } else if let Some(line) = lines.map(str::trim).find(|s| !s.is_empty()) {
            f.write_str(line)?;
        }
        Ok(())
    }

    pub fn mark(self, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
        self.level_text(f, "* ", level)
    }

    pub fn edge(self, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
        self.level_text(f, "|", level)
    }

    pub fn split(self, f: &mut fmt::Formatter, level: usize) -> fmt::Result {
        self.level_text(f, "|", level)?;
        self.level_text(f, "/", level + 1)
    }

    pub fn index(self, f: &mut fmt::Formatter, index: usize) -> fmt::Result {
        #[cfg(feature = "colored")]
        if self.colored {
            let string = index.to_string();
            return write!(f, "{}", string.yellow());
        }
        write!(f, "{index}")
    }

    pub fn at(self, f: &mut fmt::Formatter, at: At) -> fmt::Result {
        #[cfg(feature = "colored")]
        if self.colored {
            let string = alloc::format!("{}-{}", at.root, at.index);
            return write!(f, "{}", string.yellow());
        }
        write!(f, "{}-{}", at.root, at.index)
    }

    pub fn labels(
        self,
        f: &mut fmt::Formatter,
        at: At,
        head: At,
        saved: Option<At>,
    ) -> fmt::Result {
        let at_head = self.head && at == head;
        let at_saved = self.saved && matches!(saved, Some(saved) if saved == at);

        if at_head && at_saved {
            #[cfg(feature = "colored")]
            if self.colored {
                return write!(
                    f,
                    " {}{}{} {}{}",
                    "[".yellow(),
                    "HEAD".cyan(),
                    ",".yellow(),
                    "SAVED".green(),
                    "]".yellow()
                );
            }
            f.write_str(" [HEAD, SAVED]")
        } else if at_head {
            #[cfg(feature = "colored")]
            if self.colored {
                return write!(f, " {}{}{}", "[".yellow(), "HEAD".cyan(), "]".yellow());
            }
            f.write_str(" [HEAD]")
        } else if at_saved {
            #[cfg(feature = "colored")]
            if self.colored {
                return write!(f, " {}{}{}", "[".yellow(), "SAVED".green(), "]".yellow());
            }
            f.write_str(" [SAVED]")
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "std")]
    pub fn elapsed(self, f: &mut fmt::Formatter, string: String) -> fmt::Result {
        #[cfg(feature = "colored")]
        if self.colored {
            return write!(f, " {}", string.yellow());
        }
        write!(f, " {string}")
    }
}

#[cfg(feature = "colored")]
fn color_of_level(level: usize) -> Color {
    match level % 6 {
        0 => Color::Cyan,
        1 => Color::Red,
        2 => Color::Magenta,
        3 => Color::Yellow,
        4 => Color::Green,
        5 => Color::Blue,
        _ => unreachable!(),
    }
}
