#![forbid(unsafe_code)]

pub fn is_palindrome(number: u64) -> bool {
    let mut tmp = number;
    let mut number_rev = 0;
    while tmp > 0 {
        number_rev *= 10;
        number_rev += tmp % 10;
        tmp /= 10;
    }

    number == number_rev
}
