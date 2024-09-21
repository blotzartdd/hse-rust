#![forbid(unsafe_code)]

pub mod hashmap;
pub mod types;

#[cfg(test)]
mod tests {
    use std::{
        io::{stdout, Write},
        mem,
    };

    use crate::{
        hashmap::{find_keys_of_data, get_one_or_default, insert, multi_get, new_hashmap, resize},
        types::{Data, Key},
    };

    fn normalize_bucket(bucket: &mut Vec<(Key, Data)>) {
        bucket.sort();
    }

    fn normalize(table: &mut Vec<Vec<(Key, Data)>>) {
        for bucket in table {
            normalize_bucket(bucket);
        }
    }

    #[allow(unused)]
    fn print_table(table: &Vec<Vec<(Key, Data)>>) {
        println!("vec![");
        for bucket in table {
            println!("    vec![");
            for (key, data) in bucket {
                stdout().write_all(b"        (Key::new(b\"").unwrap();
                stdout().write_all(key.get()).unwrap();
                stdout().write_all(b"\"), Data::new(b\"").unwrap();
                stdout().write_all(data.get()).unwrap();
                stdout().write_all(b"\")),\n").unwrap();
            }
            println!("    ],");
        }
        println!("]");
    }

    #[test]
    fn test_new() {
        for len in [0, 1, 10, 100000] {
            let table = new_hashmap(len);
            assert_eq!(table.len(), len);
            assert!(table.iter().all(|bucket| bucket.is_empty()));
        }
    }

    #[test]
    #[should_panic(expected = "insert in empty kolhoz-table")]
    fn test_empty_insert() {
        let mut table = new_hashmap(0);
        insert(&mut table, Key::new(b"key"), Data::new(b"value"));
    }

    #[test]
    fn test_insert() {
        let mut table = new_hashmap(5);
        let entry = insert(&mut table, Key::new(b"key"), Data::new(b"value"));
        assert_eq!(*entry, Data::new(b"value"));
        assert_eq!(
            table,
            vec![
                vec![],
                vec![(Key::new(b"key"), Data::new(b"value"))],
                vec![],
                vec![],
                vec![],
            ]
        );

        let entry = insert(
            &mut table,
            Key::new(b"another-key"),
            Data::new(b"another-value"),
        );
        assert_eq!(*entry, Data::new(b"another-value"));
        assert_eq!(
            table,
            vec![
                vec![],
                vec![(Key::new(b"key"), Data::new(b"value"))],
                vec![(Key::new(b"another-key"), Data::new(b"another-value"))],
                vec![],
                vec![],
            ]
        );
    }

    #[test]
    fn test_insert_same_key() {
        let mut table = new_hashmap(5);
        for data in &["yet", "another", "kolhoz", "hash", "hash", "map"] {
            let entry = insert(&mut table, Key::new(b"key"), Data::new(data.as_bytes()));
            assert_eq!(*entry, Data::new(data.as_bytes()));
        }

        normalize(&mut table);
        assert_eq!(
            table,
            vec![
                vec![],
                vec![
                    (Key::new(b"key"), Data::new(b"another")),
                    (Key::new(b"key"), Data::new(b"hash")),
                    (Key::new(b"key"), Data::new(b"hash")),
                    (Key::new(b"key"), Data::new(b"kolhoz")),
                    (Key::new(b"key"), Data::new(b"map")),
                    (Key::new(b"key"), Data::new(b"yet")),
                ],
                vec![],
                vec![],
                vec![],
            ]
        );
    }

    #[test]
    fn test_insert_and_change() {
        let mut table = new_hashmap(5);
        let entry = insert(&mut table, Key::new(b"lol"), Data::new(b"kek"));
        assert_eq!(*entry, Data::new(b"kek"));
        *entry = Data::new(b"cheburek");
        assert_eq!(
            table,
            vec![
                vec![],
                vec![],
                vec![(Key::new(b"lol"), Data::new(b"cheburek"))],
                vec![],
                vec![],
            ]
        );
    }

    #[test]
    fn test_get_one_empty() {
        let table = new_hashmap(0);
        let default = Data::new(b"default");
        let key = Key::new(b"two");
        let result = get_one_or_default(&table, &key, &default);
        assert_eq!(*result, default);
    }

    #[test]
    fn test_get_one() {
        let mut table = new_hashmap(5);
        insert(&mut table, Key::new(b"one"), Data::new(b"odin"));
        let dva = insert(&mut table, Key::new(b"two"), Data::new(b"two"));
        *dva = Data::new(b"dva");

        let default = Data::new(b"default");

        let result;
        {
            let key = Key::new(b"one");
            result = get_one_or_default(&table, &key, &default);
        }
        assert_eq!(*result, Data::new(b"odin"));

        let result;
        {
            let key = Key::new(b"two");
            result = get_one_or_default(&table, &key, &default);
        }
        assert_eq!(*result, Data::new(b"dva"));

        let result;
        {
            let key = Key::new(b"three");
            result = get_one_or_default(&table, &key, &default);
        }
        assert_eq!(*result, default);
    }

    #[test]
    fn test_get_one_with_multiple_options() {
        let mut table = new_hashmap(5);
        let datas: [&[u8]; 3] = [b"odin", b"uno", b"ber"];
        for data in datas {
            insert(&mut table, Key::new(b"one"), Data::new(data));
        }

        let default = Data::new(b"default");
        let result;
        {
            let key = Key::new(b"one");
            result = get_one_or_default(&table, &key, &default);
        }
        assert!(datas.contains(&result.get().as_slice()));
    }

    #[test]
    fn test_mutli_get_empty() {
        let table = new_hashmap(0);
        let keys = vec![Key::new(b"key"), Key::new(b""), Key::new(b"x")];
        let result = multi_get(&table, &keys);
        assert_eq!(
            result,
            vec![
                (&Key::new(b"key"), vec![]),
                (&Key::new(b""), vec![]),
                (&Key::new(b"x"), vec![]),
            ]
        );
    }

    #[test]
    fn test_mutli_get_nokeys() {
        let mut table = new_hashmap(5);
        insert(&mut table, Key::new(b"1"), Data::new(b"I"));
        insert(&mut table, Key::new(b"2"), Data::new(b"II"));
        assert!(multi_get(&table, &Vec::new()).is_empty());
    }

    #[test]
    fn test_mutli_get() {
        let mut table = new_hashmap(5);
        insert(&mut table, Key::new(b"1"), Data::new(b"I"));
        insert(&mut table, Key::new(b"2"), Data::new(b"II"));

        let datas: [&[u8]; 3] = [b"odin", b"uno", b"ber"];
        for data in datas {
            insert(&mut table, Key::new(b"one"), Data::new(data));
        }

        insert(&mut table, Key::new(b"3"), Data::new(b"three"));
        insert(&mut table, Key::new(b"3"), Data::new(b"III"));
        insert(&mut table, Key::new(b"4"), Data::new(b"IV"));
        insert(&mut table, Key::new(b"5"), Data::new(b"V"));
        insert(&mut table, Key::new(b"6"), Data::new(b"VI"));
        insert(&mut table, Key::new(b"6"), Data::new(b"six"));

        let normalize_result = |result: &mut Vec<(&Key, Vec<&Data>)>| {
            for (_, datas) in result {
                datas.sort();
            }
        };

        // table lives longer than lookup keys
        {
            let keys = vec![
                Key::new(b"6"),
                Key::new(b"ten"),
                Key::new(b""),
                Key::new(b"3"),
                Key::new(b"one"),
            ];
            let mut result = multi_get(&table, &keys);
            normalize_result(&mut result);
            assert_eq!(
                result,
                vec![
                    (&Key::new(b"6"), vec![&Data::new(b"VI"), &Data::new(b"six")]),
                    (&Key::new(b"ten"), vec![]),
                    (&Key::new(b""), vec![]),
                    (
                        &Key::new(b"3"),
                        vec![&Data::new(b"III"), &Data::new(b"three")]
                    ),
                    (
                        &Key::new(b"one"),
                        vec![&Data::new(b"ber"), &Data::new(b"odin"), &Data::new(b"uno")]
                    ),
                ]
            );
            let (_, result_data): (Vec<&Key>, Vec<Vec<&Data>>) = result.into_iter().unzip();
            mem::drop(keys);
            assert_eq!(
                result_data,
                vec![
                    vec![&Data::new(b"VI"), &Data::new(b"six")],
                    vec![],
                    vec![],
                    vec![&Data::new(b"III"), &Data::new(b"three")],
                    vec![&Data::new(b"ber"), &Data::new(b"odin"), &Data::new(b"uno")],
                ]
            );
        }

        // keys live longer than table and its data
        {
            let keys = vec![
                Key::new(b"6"),
                Key::new(b"ten"),
                Key::new(b""),
                Key::new(b"3"),
                Key::new(b"one"),
            ];
            let result = multi_get(&table, &keys);
            let (result_keys, _): (Vec<&Key>, Vec<Vec<&Data>>) = result.into_iter().unzip();
            mem::drop(table);
            assert_eq!(result_keys, keys.iter().collect::<Vec<&Key>>());
        }
    }

    #[test]
    fn test_find_keys_of_data_empty() {
        let table = new_hashmap(0);
        let data = Data::new(b"key");
        let keys = find_keys_of_data(&table, &data);
        assert!(keys.is_empty());
    }

    #[test]
    fn test_find_keys_of_data() {
        let mut table = new_hashmap(3);
        insert(&mut table, Key::new(b"1"), Data::new(b"I"));
        insert(&mut table, Key::new(b"three"), Data::new(b"III"));
        insert(&mut table, Key::new(b"odin"), Data::new(b"I"));
        insert(&mut table, Key::new(b"one"), Data::new(b"I"));
        insert(&mut table, Key::new(b"och"), Data::new(b"III"));
        insert(&mut table, Key::new(b"2"), Data::new(b"II"));
        insert(&mut table, Key::new(b"four"), Data::new(b"VI"));
        insert(&mut table, Key::new(b"dva"), Data::new(b"II"));
        insert(&mut table, Key::new(b"ike"), Data::new(b"II"));

        let data = Data::new(b"nineeleven");
        let keys = find_keys_of_data(&table, &data);
        assert!(keys.is_empty());

        let data = Data::new(b"II");
        let mut keys = find_keys_of_data(&table, &data);
        mem::drop(data);
        keys.sort();
        assert_eq!(
            keys,
            vec![&Key::new(b"2"), &Key::new(b"dva"), &Key::new(b"ike")]
        );
    }

    fn create_test_table(n: usize) -> Vec<Vec<(Key, Data)>> {
        let raw = [
            (Key::new(b"1"), Data::new(b"one")),
            (Key::new(b"5"), Data::new(b"V")),
            (Key::new(b"5"), Data::new(b"five")),
            (Key::new(b"2"), Data::new(b"two")),
            (Key::new(b"3"), Data::new(b"three")),
            (Key::new(b"6"), Data::new(b"VI")),
            (Key::new(b"4"), Data::new(b"four")),
            (Key::new(b"3"), Data::new(b"III")),
            (Key::new(b"3"), Data::new(b"three")),
        ];

        let mut table = new_hashmap(n);
        if n > 0 {
            for (key, data) in raw {
                insert(&mut table, key, data);
            }
        }
        normalize(&mut table);
        table
    }

    #[test]
    fn test_resize_same_size() {
        let mut table = create_test_table(3);
        assert_eq!(
            table,
            vec![
                vec![
                    (Key::new(b"2"), Data::new(b"two")),
                    (Key::new(b"4"), Data::new(b"four")),
                ],
                vec![
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                    (Key::new(b"6"), Data::new(b"VI")),
                ],
                vec![(Key::new(b"1"), Data::new(b"one"))],
            ]
        );

        resize(&mut table, 3);
        normalize(&mut table);
        assert_eq!(
            table,
            vec![
                vec![
                    (Key::new(b"2"), Data::new(b"two")),
                    (Key::new(b"4"), Data::new(b"four")),
                ],
                vec![
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                    (Key::new(b"6"), Data::new(b"VI")),
                ],
                vec![(Key::new(b"1"), Data::new(b"one"))],
            ]
        );
    }

    #[test]
    fn test_resize_empty() {
        let mut table = create_test_table(0);
        assert!(table.is_empty());

        resize(&mut table, 3);
        assert_eq!(table, vec![vec![], vec![], vec![]],);

        resize(&mut table, 1);
        assert_eq!(table, vec![vec![]],);

        resize(&mut table, 0);
        assert!(table.is_empty());
    }

    #[test]
    fn test_resize_expand_and_shrink() {
        let mut table = create_test_table(3);
        assert_eq!(
            table,
            vec![
                vec![
                    (Key::new(b"2"), Data::new(b"two")),
                    (Key::new(b"4"), Data::new(b"four")),
                ],
                vec![
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                    (Key::new(b"6"), Data::new(b"VI")),
                ],
                vec![(Key::new(b"1"), Data::new(b"one"))],
            ]
        );

        resize(&mut table, 4);
        normalize(&mut table);
        assert_eq!(
            table,
            vec![
                vec![
                    (Key::new(b"1"), Data::new(b"one")),
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                ],
                vec![(Key::new(b"6"), Data::new(b"VI"))],
                vec![],
                vec![
                    (Key::new(b"2"), Data::new(b"two")),
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"4"), Data::new(b"four")),
                ],
            ]
        );

        resize(&mut table, 11);
        normalize(&mut table);
        assert_eq!(
            table,
            vec![
                vec![],
                vec![(Key::new(b"4"), Data::new(b"four"))],
                vec![
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                ],
                vec![],
                vec![],
                vec![(Key::new(b"6"), Data::new(b"VI"))],
                vec![(Key::new(b"1"), Data::new(b"one"))],
                vec![(Key::new(b"2"), Data::new(b"two"))],
                vec![],
                vec![
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                ],
                vec![],
            ]
        );

        resize(&mut table, 100);
        resize(&mut table, 2);
        resize(&mut table, 11);

        resize(&mut table, 5);
        normalize(&mut table);
        assert_eq!(
            table,
            vec![
                vec![(Key::new(b"6"), Data::new(b"VI"))],
                vec![(Key::new(b"4"), Data::new(b"four"))],
                vec![],
                vec![
                    (Key::new(b"3"), Data::new(b"III")),
                    (Key::new(b"3"), Data::new(b"three")),
                    (Key::new(b"3"), Data::new(b"three")),
                ],
                vec![
                    (Key::new(b"1"), Data::new(b"one")),
                    (Key::new(b"2"), Data::new(b"two")),
                    (Key::new(b"5"), Data::new(b"V")),
                    (Key::new(b"5"), Data::new(b"five")),
                ],
            ]
        );

        resize(&mut table, 1);
        normalize(&mut table);
        assert_eq!(
            table,
            vec![vec![
                (Key::new(b"1"), Data::new(b"one")),
                (Key::new(b"2"), Data::new(b"two")),
                (Key::new(b"3"), Data::new(b"III")),
                (Key::new(b"3"), Data::new(b"three")),
                (Key::new(b"3"), Data::new(b"three")),
                (Key::new(b"4"), Data::new(b"four")),
                (Key::new(b"5"), Data::new(b"V")),
                (Key::new(b"5"), Data::new(b"five")),
                (Key::new(b"6"), Data::new(b"VI")),
            ]]
        );

        resize(&mut table, 0);
        assert!(table.is_empty());

        resize(&mut table, 100);
        assert_eq!(table.len(), 100);
        assert!(table.iter().all(|b| b.is_empty()));
    }
}
