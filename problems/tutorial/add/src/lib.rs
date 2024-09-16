#![forbid(unsafe_code)]

pub fn add(x: i32, y: i32) -> i32 {
    if x > 0 && y > i32::MAX - x {
        i32::MAX
    } else if x < 0 && y < i32::MIN - x {
        i32::MIN
    } else {
        x + y
    }
}
