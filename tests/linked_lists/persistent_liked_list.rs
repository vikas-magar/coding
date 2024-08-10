use coding_lib::linked_lists::persistent_linked_list::List;

#[test]
fn basic() {
    let ll = List::new();

    assert_eq!(ll.head(), None);
    let ll = ll.prepend(1).prepend(2).prepend(3);

    assert_eq!(ll.head(), Some(&3));

    let ll = ll.tail();
    assert_eq!(ll.head(), Some(&2))
}

#[test]
fn iter() {
    let ll = List::new().prepend(12).prepend(13).prepend(14);

    let mut iter = ll.iter();
    assert_eq!(iter.next(), Some(&14))
}
