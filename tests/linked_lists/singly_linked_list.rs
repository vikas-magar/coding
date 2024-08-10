use coding_lib::linked_lists::singly_linked_list::List;

#[test]
fn test_one() {
    let input = vec![1, 4, 2, 3];
    let mut ll = List::new();
    assert_eq!(ll.pop(), None);
    for ele in input {
        ll.push(ele);
    }

    assert_eq!(ll.pop(), Some(3));
}

#[test]
fn peek() {
    let mut ll = List::new();
    assert_eq!(ll.peek(), None);
    assert_eq!(ll.peek_mut(), None);
    ll.push(1);
    ll.push(2);
    ll.push(3);

    assert_eq!(ll.peek(), Some(&3));
    assert_eq!(ll.peek_mut(), Some(&mut 3));

    ll.peek_mut().map(|value| *value = 42);

    assert_eq!(ll.peek(), Some(&42));
    assert_eq!(ll.pop(), Some(42));
}
#[test]
fn into_iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}
#[test]
fn iter() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}

#[test]
fn iter_mut() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 1));
}
