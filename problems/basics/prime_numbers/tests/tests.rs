use prime_numbers::get_n_prime_numbers;

#[test]
fn it_works() {
    assert_eq!(get_n_prime_numbers(0), []);
    assert_eq!(get_n_prime_numbers(1), [2]);
    assert_eq!(get_n_prime_numbers(2), [2, 3]);
    assert_eq!(get_n_prime_numbers(3), [2, 3, 5]);
    assert_eq!(
        get_n_prime_numbers(46),
        [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199
        ]
    );
    assert_eq!(
        get_n_prime_numbers(10_000)[10_000 - 11..],
        [104659, 104677, 104681, 104683, 104693, 104701, 104707, 104711, 104717, 104723, 104729]
    );
}
