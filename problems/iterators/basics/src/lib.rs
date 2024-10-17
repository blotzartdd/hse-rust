#![forbid(unsafe_code)]

pub fn add2(iterator: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    iterator.map(|x| x + 2)
}

pub fn div3() -> impl Iterator<Item = i32> {
    (1..).filter(|x| x % 3 == 0)
}

pub fn take_n(iterator: impl Iterator<Item = i32>, n: usize) -> Vec<i32> {
    iterator.take(n).collect()
}
