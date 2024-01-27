use heapless::String;
use undo::{Add, Record};
fn main() {
    let mut target = String::<256>::new();
    let mut record = Record::<_, 32>::new();

    record.edit(&mut target, Add('a'));
    record.edit(&mut target, Add('b'));
    record.edit(&mut target, Add('c'));
    record.edit(&mut target, Add('d'));
    record.edit(&mut target, Add('e'));
    record.edit(&mut target, Add('f'));
    assert_eq!(target, "abcdef");

    record.set_saved();

    record.undo(&mut target);
    record.undo(&mut target);
    assert_eq!(target, "abcd");

    println!("{}", record.display::<256>());
}
