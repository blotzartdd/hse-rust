#![forbid(unsafe_code)]

pub fn flatten<const N: usize>(data: Vec<Box<[&mut i32; N]>>) -> Vec<&mut i32> {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn transform_to_fixed_arr<const N: usize>(data: &mut Vec<Vec<i32>>) -> Vec<Box<[&mut i32; N]>> {
    // TODO: your code goes here.
    unimplemented!()
}
