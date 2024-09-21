use flatten::{flatten, transform_to_fixed_arr};

fn transform_arr_ok<const N: usize>(
    before: &Vec<Vec<i32>>,
    after: &Vec<Box<[&mut i32; N]>>,
) -> bool {
    if before.len() != after.len() {
        return false;
    }

    for (row_before, row_after) in before.iter().zip(after.iter()) {
        if row_before.len() != row_after.len() {
            return false;
        }

        for (value_before, value_after) in row_before.iter().zip(row_after.iter()) {
            if value_before != *value_after {
                return false;
            }
        }
    }

    true
}

fn flatten_arr_ok(before: &Vec<Vec<i32>>, after: &Vec<&mut i32>) -> bool {
    if before.is_empty() {
        return after.is_empty();
    }

    assert_eq!(before.len() * before[0].len(), after.len());
    for i in 0..before.len() {
        for j in 0..before[i].len() {
            if before[i][j] != *after[i * before[i].len() + j] {
                return false;
            }
        }
    }
    true
}

#[test]
fn it_works() {
    let mut v = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4], vec![10, 3, 5, 2]];
    let v_cloned = v.clone();
    let x = transform_to_fixed_arr::<4>(&mut v);
    assert!(transform_arr_ok(&v_cloned, &x));
    let flatten = flatten(x);
    assert!(flatten_arr_ok(&v_cloned, &flatten));
}

#[test]
#[should_panic(expected = "Inner vectors are of different size")]
fn test_invalid_sizes() {
    let mut v = vec![vec![1, 2, 3, 4], vec![1, 2, 3, 4, 5]];
    let _ = transform_to_fixed_arr::<4>(&mut v);
}

#[test]
fn test_empty_input() {
    let mut v = vec![vec![], vec![]];
    let v_cloned = v.clone();
    let x = transform_to_fixed_arr::<0>(&mut v);
    assert!(transform_arr_ok(&v_cloned, &x));
    assert!(flatten_arr_ok(&v_cloned, &flatten(x)));

    let mut v = vec![];
    let v_cloned = v.clone();
    let x = transform_to_fixed_arr::<0>(&mut v);
    assert!(transform_arr_ok(&v_cloned, &x));
    assert!(flatten_arr_ok(&v_cloned, &flatten(x)));

    let mut v = vec![];
    let v_cloned = v.clone();
    let x = transform_to_fixed_arr::<10>(&mut v);
    assert!(transform_arr_ok(&v_cloned, &x));
    assert!(flatten_arr_ok(&v_cloned, &flatten(x)));
}

#[test]
fn test_big_arrays() {
    macro_rules! big_array_test {
        ($inner_size:literal, $outer_size:literal) => {
            let mut v = vec![vec![0; $inner_size]; $outer_size];
            let v_cloned = v.clone();
            let x = transform_to_fixed_arr::<{ $inner_size }>(&mut v);
            assert!(transform_arr_ok(&v_cloned, &x));
            assert!(flatten_arr_ok(&v_cloned, &flatten(x)));
        };
    }

    big_array_test!(1000, 10);
    big_array_test!(100, 100);
    big_array_test!(10, 1000);
}
