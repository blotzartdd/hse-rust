#![forbid(unsafe_code)]

pub fn get_nth_fibonacci(n: u32) -> u32 {
    let mut f0 = 0;
    let mut f1 = 1;
    if n == 0 {
        return f0;
    } else if n == 1 {
        return f1;
    }

    for _ in 2..n + 1 {
        let f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }

    f1
}
