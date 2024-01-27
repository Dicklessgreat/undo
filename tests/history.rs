use heapless::String;
use undo::{Add, At, History};

const A: Add<256> = Add('a');
const B: Add<256> = Add('b');
const C: Add<256> = Add('c');
const D: Add<256> = Add('d');
const E: Add<256> = Add('e');
const F: Add<256> = Add('f');
const G: Add<256> = Add('g');
const H: Add<256> = Add('h');
const I: Add<256> = Add('i');
const J: Add<256> = Add('j');
const K: Add<256> = Add('k');
const L: Add<256> = Add('l');
const M: Add<256> = Add('m');
const N: Add<256> = Add('n');
const O: Add<256> = Add('o');
const P: Add<256> = Add('p');
const Q: Add<256> = Add('q');

#[test]
fn go_to() {
    //          m
    //          |
    //    j  k  l
    //     \ | /
    //       i
    //       |
    // e  g  h
    // |  | /
    // d  f  p - q *
    // | /  /
    // c  n - o
    // | /
    // b
    // |
    // a
    let mut target = String::<256>::new();
    let mut history = History::<_, 32>::new();
    history.edit(&mut target, A);
    history.edit(&mut target, B);
    history.edit(&mut target, C);
    history.edit(&mut target, D);
    history.edit(&mut target, E);
    assert_eq!(target, "abcde");
    history.undo(&mut target).unwrap();
    history.undo(&mut target).unwrap();
    assert_eq!(target, "abc");
    let abc = history.head();

    history.edit(&mut target, F);
    history.edit(&mut target, G);
    assert_eq!(target, "abcfg");
    let abcfg = history.head();

    history.undo(&mut target).unwrap();
    history.edit(&mut target, H);
    history.edit(&mut target, I);
    history.edit(&mut target, J);
    assert_eq!(target, "abcfhij");
    let abcfhij = history.head();

    history.undo(&mut target).unwrap();
    history.edit(&mut target, K);
    assert_eq!(target, "abcfhik");
    let abcfhik = history.head();

    history.undo(&mut target).unwrap();
    history.edit(&mut target, L);
    assert_eq!(target, "abcfhil");
    history.edit(&mut target, M);
    assert_eq!(target, "abcfhilm");
    let abcfhilm = history.head();
    history.go_to(&mut target, At::new(abc.root, 2));
    history.edit(&mut target, N);
    history.edit(&mut target, O);
    assert_eq!(target, "abno");
    let abno = history.head();

    history.undo(&mut target).unwrap();
    history.edit(&mut target, P);
    history.edit(&mut target, Q);
    assert_eq!(target, "abnpq");

    let abnpq = history.head();
    history.go_to(&mut target, abc);
    assert_eq!(target, "abc");
    history.go_to(&mut target, abcfg);
    assert_eq!(target, "abcfg");
    history.go_to(&mut target, abcfhij);
    assert_eq!(target, "abcfhij");
    history.go_to(&mut target, abcfhik);
    assert_eq!(target, "abcfhik");
    history.go_to(&mut target, abcfhilm);
    assert_eq!(target, "abcfhilm");
    history.go_to(&mut target, abno);
    assert_eq!(target, "abno");
    history.go_to(&mut target, abnpq);
    assert_eq!(target, "abnpq");
}

#[test]
fn checkpoint() {
    let mut target = String::<256>::new();
    let mut history = History::<_, 32>::new();
    let mut checkpoint = history.checkpoint::<16>();

    checkpoint.edit(&mut target, A);
    checkpoint.edit(&mut target, B);
    checkpoint.edit(&mut target, C);
    assert_eq!(target, "abc");

    checkpoint.undo(&mut target);
    checkpoint.undo(&mut target);
    assert_eq!(target, "a");

    checkpoint.edit(&mut target, D);
    checkpoint.edit(&mut target, E);
    assert_eq!(target, "ade");

    checkpoint.cancel(&mut target);
    assert_eq!(target, "");
}

#[test]
fn next_and_prev() {
    let mut target = String::new();
    let mut history = History::<_, 32>::new();
    history.edit(&mut target, A);
    history.edit(&mut target, B);
    history.edit(&mut target, C);
    assert_eq!(history.next_branch_head(), None);
    assert_eq!(history.prev_branch_head(), None);

    history.undo(&mut target).unwrap();
    history.undo(&mut target).unwrap();
    history.edit(&mut target, D);
    history.edit(&mut target, E);
    assert_eq!(history.next_branch_head(), None);
    let prev_head = history.prev_branch_head().unwrap();
    assert_eq!(prev_head, At::new(0, 2));

    history.go_to(&mut target, prev_head);
    assert_eq!(history.next_branch_head(), Some(At::new(1, 2)));
    assert_eq!(history.prev_branch_head(), None);
}
