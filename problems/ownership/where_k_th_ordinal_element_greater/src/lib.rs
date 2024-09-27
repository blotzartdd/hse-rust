#![forbid(unsafe_code)]

pub fn where_k_th_ordinal_element_greater<'a>(
    lhs: &'a Vec<i32>,
    rhs: &'a Vec<i32>,
    k: usize,
) -> &'a Vec<i32> {
    let mut l = lhs.clone();
    let mut r = rhs.clone();

    l.sort();
    r.sort();

    if l[k] >= r[k] {
        lhs
    } else {
        rhs
    }
}
