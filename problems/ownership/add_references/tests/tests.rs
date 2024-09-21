use add_references::{calculate_statistics, Statistics};

#[test]
fn it_works() {
    let mut v = vec![0];
    assert_eq!(
        Statistics {
            average: 0,
            median: 0,
            min: 0,
            max: 0,
        },
        calculate_statistics(&v)
    );

    v.extend_from_slice(&[2, 3, 4, 1000]);
    assert_eq!(
        Statistics {
            average: 201,
            median: 3,
            min: 0,
            max: 1000,
        },
        calculate_statistics(&v)
    );

    v.extend_from_slice(&[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, -1, 1001]);
    assert_eq!(
        Statistics {
            average: 101,
            median: 1,
            min: -1,
            max: 1001,
        },
        calculate_statistics(&v)
    );
}
