use minmax_queue::MinMaxQueue;

#[test]
fn test_empty() {
    let mut queue = MinMaxQueue::new();
    assert!(queue.is_empty());
    queue.push(10);
    assert!(!queue.is_empty());
}

#[test]
fn test_push() {
    let mut queue = MinMaxQueue::new();
    assert_eq!(None, queue.min());
    assert_eq!(None, queue.max());
    assert_eq!(0, queue.len());
    queue.push(0);
    assert_eq!(Some(0), queue.min());
    assert_eq!(Some(0), queue.max());
    assert_eq!(Some(0), queue.last());
    assert_eq!(1, queue.len());
    queue.push(1);
    assert_eq!(Some(0), queue.min());
    assert_eq!(Some(1), queue.max());
    assert_eq!(Some(1), queue.last());
    assert_eq!(2, queue.len());
    queue.push(2);
    assert_eq!(Some(0), queue.min());
    assert_eq!(Some(2), queue.max());
    assert_eq!(Some(2), queue.last());
    assert_eq!(3, queue.len());
    queue.push(3);
    assert_eq!(Some(0), queue.min());
    assert_eq!(Some(3), queue.max());
    assert_eq!(Some(3), queue.last());
    assert_eq!(4, queue.len());
    queue.push(4);
    assert_eq!(Some(0), queue.min());
    assert_eq!(Some(4), queue.max());
    assert_eq!(Some(4), queue.last());
    assert_eq!(5, queue.len());

    assert_eq!(Some(0), queue.first());
    assert_eq!(Some(0), queue.pop());
    assert_eq!(Some(1), queue.min());
    assert_eq!(Some(4), queue.max());
    assert_eq!(4, queue.len());
    assert_eq!(Some(1), queue.first());
    assert_eq!(Some(1), queue.pop());
    assert_eq!(Some(2), queue.min());
    assert_eq!(Some(4), queue.max());
    assert_eq!(3, queue.len());
    assert_eq!(Some(2), queue.first());
    assert_eq!(Some(2), queue.pop());
    assert_eq!(Some(3), queue.min());
    assert_eq!(Some(4), queue.max());
    assert_eq!(2, queue.len());
    assert_eq!(Some(3), queue.first());
    assert_eq!(Some(3), queue.pop());
    assert_eq!(Some(4), queue.min());
    assert_eq!(Some(4), queue.max());
    assert_eq!(1, queue.len());
    assert_eq!(Some(4), queue.first());
    assert_eq!(Some(4), queue.pop());
    assert_eq!(0, queue.len());
    assert_eq!(None, queue.first());
    assert_eq!(None, queue.pop());
    assert_eq!(None, queue.min());
    assert_eq!(None, queue.max());
}

#[test]
fn test_large() {
    let mut queue = MinMaxQueue::new();

    for i in 0..1000000 {
        queue.push(i);
        assert_eq!(Some(i), queue.max());
    }

    for i in 0..1000000 {
        assert_eq!(Some(i), queue.min());
        queue.pop();
    }
}
