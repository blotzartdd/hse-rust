#![forbid(unsafe_code)]

use std::convert::TryInto;

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    let mut res: Vec<&mut i32> = Vec::new();
    for box_ in data {
        for num in *box_ {
            res.push(num);
        }
    }

    res
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut [Vec<i32>]) -> Vec<Box<[&mut i32; N]>> {
    let mut fixed_arr: Vec<Box<[&mut i32; N]>> = Vec::new();
    for v in data.iter_mut() {
        if v.len() != N {
            panic!("Inner vectors are of different size");
        }

        let mut new_v: Vec<&mut i32> = vec![];
        for el in v.iter_mut() {
            new_v.push(el);
        }

        let array: [&mut i32; N] =
            <Vec<&mut i32> as TryInto<[&mut i32; N]>>::try_into(new_v).unwrap();
        fixed_arr.push(Box::new(array));
    }

    fixed_arr
}
