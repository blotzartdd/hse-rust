#![forbid(unsafe_code)]

use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn insert(table: &mut Vec<Vec<(Key, Data)>>, key: Key, value: Data) -> &mut Data {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn get_one_or_default(table: &Vec<Vec<(Key, Data)>>, key: &Key, default_value: &Data) -> &Data {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn multi_get(table: &Vec<Vec<(Key, Data)>>, keys: &Vec<Key>) -> Vec<(&Key, Vec<&Data>)> {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn find_keys_of_data(table: &Vec<Vec<(Key, Data)>>, value: &Data) -> Vec<&Key> {
    // TODO: your code goes here.
    unimplemented!()
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    // TODO: your code goes here.
    unimplemented!()
}
