use chrono::{DateTime, Local};
use heapless::String;
use std::time::SystemTime;
use std::{fmt::Write, io};
use undo::{Add, At, History};

fn custom_st_fmt<const SIZE: usize>(_: SystemTime, at: SystemTime) -> String<SIZE> {
    let mut result = String::<SIZE>::new();
    let dt = DateTime::<Local>::from(at);
    result
        .write_fmt(format_args!("{}", dt.format("%H:%M:%S").to_string()))
        .expect("enough space");
    result
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut target = heapless::String::<256>::new();
    let mut history = History::<_, 32>::builder().limit(10).build();

    loop {
        println!(
            "Enter a string. Use '<' to undo, '>' to redo, '*' to save, '?' to revert to save, '+' to switch to next branch, '-' to switch to previous branch, and '! i-j' for goto: "
        );
        let mut buf = std::string::String::new();
        let n = stdin.read_line(&mut buf)?;
        if n == 0 {
            return Ok(());
        }

        // Clears the terminal.
        print!("{}c", 27 as char);

        let mut chars = buf.trim().chars();
        while let Some(c) = chars.next() {
            match c {
                '!' => {
                    let tail = chars.collect::<String<256>>();
                    let mut at = tail
                        .trim()
                        .split('-')
                        .filter_map(|n| n.parse::<usize>().ok());

                    let root = at.next().unwrap_or_default();
                    let index = at.next().unwrap_or_default();
                    history.go_to(&mut target, At::new(root, index));
                    break;
                }
                '<' => {
                    history.undo(&mut target);
                }
                '>' => {
                    history.redo(&mut target);
                }
                '*' => {
                    history.set_saved();
                }
                '?' => {
                    history.revert(&mut target);
                }
                '+' => {
                    if let Some(at) = history.next_branch_head() {
                        history.go_to(&mut target, at);
                    }
                }
                '-' => {
                    if let Some(at) = history.prev_branch_head() {
                        history.go_to(&mut target, at);
                    }
                }
                c => {
                    history.edit(&mut target, Add(c));
                }
            }
        }

        println!("{}\n", history.display::<256>().set_st_fmt(&custom_st_fmt));
        println!("Target: {target}");
    }
}
