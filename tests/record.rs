use heapless::String;
use undo::{Add, Record};

const A: Add<256> = Add('a');
const B: Add<256> = Add('b');
const C: Add<256> = Add('c');
const D: Add<256> = Add('d');
const E: Add<256> = Add('e');
const F: Add<256> = Add('f');

#[test]
fn go_to() {
    let mut target = String::new();
    let mut record = Record::<_, 32>::new();
    record.edit(&mut target, A);
    record.edit(&mut target, B);
    record.edit(&mut target, C);
    record.edit(&mut target, D);
    record.edit(&mut target, E);

    record.go_to(&mut target, 0);
    assert_eq!(record.head(), 0);
    assert_eq!(target, "");
    record.go_to(&mut target, 5);
    assert_eq!(record.head(), 5);
    assert_eq!(target, "abcde");
    record.go_to(&mut target, 1);
    assert_eq!(record.head(), 1);
    assert_eq!(target, "a");
    record.go_to(&mut target, 4);
    assert_eq!(record.head(), 4);
    assert_eq!(target, "abcd");
    record.go_to(&mut target, 2);
    assert_eq!(record.head(), 2);
    assert_eq!(target, "ab");
    record.go_to(&mut target, 3);
    assert_eq!(record.head(), 3);
    assert_eq!(target, "abc");
    assert!(record.go_to(&mut target, 6).is_empty());
    assert_eq!(record.head(), 3);
}

#[test]
fn edits() {
    let mut target = String::new();
    let mut record = Record::<_, 32>::new();
    record.edit(&mut target, A);
    record.edit(&mut target, B);
    let collected = record.entries().map(AsRef::as_ref).collect::<Vec<_>>();
    assert_eq!(&collected[..], &[&A, &B][..]);
}

#[test]
fn checkpoint_saved() {
    let mut target = String::new();
    let mut record = Record::<_, 32>::new();
    record.edit(&mut target, A);
    record.edit(&mut target, B);
    record.edit(&mut target, C);
    record.set_saved();
    record.undo(&mut target).unwrap();
    record.undo(&mut target).unwrap();
    record.undo(&mut target).unwrap();
    let mut cp = record.checkpoint::<16>();
    cp.edit(&mut target, D);
    cp.edit(&mut target, E);
    cp.edit(&mut target, F);
    assert_eq!(target, "def");
    cp.cancel(&mut target);
    assert_eq!(target, "");
    record.redo(&mut target).unwrap();
    record.redo(&mut target).unwrap();
    record.redo(&mut target).unwrap();
    assert!(record.is_saved());
    assert_eq!(target, "abc");
}
