use fibonacci::get_nth_fibonacci;

#[test]
fn it_works() {
    assert_eq!(get_nth_fibonacci(0), 0);
    assert_eq!(get_nth_fibonacci(1), 1);
    assert_eq!(get_nth_fibonacci(2), 1);
    assert_eq!(get_nth_fibonacci(3), 2);
    assert_eq!(get_nth_fibonacci(4), 3);
    assert_eq!(get_nth_fibonacci(5), 5);
    assert_eq!(get_nth_fibonacci(6), 8);
    assert_eq!(get_nth_fibonacci(7), 13);
    assert_eq!(get_nth_fibonacci(8), 21);
    assert_eq!(get_nth_fibonacci(9), 34);
    assert_eq!(get_nth_fibonacci(10), 55);
}
