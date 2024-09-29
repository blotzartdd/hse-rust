#![forbid(unsafe_code)]
use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    let mut table: Vec<Vec<(Key, Data)>> = Vec::new();
    for _ in 0..len {
        let v: Vec<(Key, Data)> = Vec::new();
        table.push(v);
    }
    table
}

pub fn insert(table: &mut [Vec<(Key, Data)>], key: Key, value: Data) -> &mut Data {
    if table.is_empty() {
        panic!("insert in empty kolhoz-table");
    }

    let h = key.get_hash() as usize;
    let n = table.len();
    table[h % n].push((key, value));

    let last_index = table[h % n].len() - 1;
    &mut table[h % n][last_index].1
}

pub fn get_one_or_default<'a>(
    table: &'a [Vec<(Key, Data)>],
    key: &Key,
    default_value: &'a Data,
) -> &'a Data {
    if !table.is_empty() {
        let bucket_index = key.get_hash() as usize % table.len();
        for opt in table[bucket_index].iter() {
            if opt.0 == *key {
                return &opt.1;
            }
        }
    }

    default_value
}

pub fn multi_get<'table, 'keys>(
    table: &'table [Vec<(Key, Data)>],
    keys: &'keys [Key],
) -> Vec<(&'keys Key, Vec<&'table Data>)> {
    let mut res: Vec<(&Key, Vec<&Data>)> = Vec::new();
    let n = table.len();

    for key in keys.iter() {
        let mut values: Vec<&Data> = vec![];

        if n != 0 {
            let bucket_index = key.get_hash() as usize % n;
            for opt in table[bucket_index].iter() {
                if opt.0 == *key {
                    values.push(&opt.1);
                }
            }
        }

        res.push((key, values));
    }

    res
}

pub fn find_keys_of_data<'a>(table: &'a Vec<Vec<(Key, Data)>>, value: &Data) -> Vec<&'a Key> {
    let mut res: Vec<&Key> = Vec::new();
    for v in table {
        for pair in v {
            if pair.1 == *value {
                res.push(&pair.0);
            }
        }
    }

    res
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    let mut new_table = new_hashmap(new_len);

    if new_len != 0 {
        for v in table.iter_mut() {
            while let Some((key, data)) = v.pop() {
                let new_index = key.get_hash() as usize % new_len;
                new_table[new_index].push((key, data));
            }
        }
    }

    *table = new_table;
}
