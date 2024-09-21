use where_k_th_ordinal_element_greater::where_k_th_ordinal_element_greater;

#[test]
fn small() {
    let lhs = vec![3, 0, 4];
    let rhs = vec![1, 2, 5];

    assert_eq!(&rhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 0usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 1usize));
    assert_eq!(&rhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 2usize));
}

#[test]
fn big() {
    let lhs = vec![
        52, 40, 37, 55, 93, 27, 40, 2, 12, 69, 98, 29, 91, 82, 3, 99, 90, 28, 27, 11, 65, 57, 21,
        36, 67, 37, 50, 91, 81, 11, 2, 40, 87, 34, 79, 34, 56, 62, 64, 78, 35, 40, 75, 85, 48, 60,
        36, 14, 83, 42, 80, 37, 29, 62, 17, 75, 55, 89, 45, 60, 97, 15, 93, 94, 72, 44, 50, 12, 47,
        92, 93, 11, 53, 94, 94, 17, 35, 52, 15, 78, 78, 43, 31, 74, 91, 4, 77, 31, 63, 96, 66, 40,
        22, 75, 6, 91, 3, 71, 32, 73,
    ];
    let rhs = vec![
        139, 39, 134, -42, -16, 87, -99, 76, -83, -72, -13, 73, -36, 163, -82, -13, 90, 192, -52,
        147, 47, -6, 108, 48, 78, -90, -94, -51, 10, 95, 25, -2, 24, 2, -64, 199, 23, 91, -80, -75,
        161, 165, -24, 98, 120, -17, 76, -81, -60, 155, -49, 139, 92, 130, -27, 147, 84, 178, 21,
        98, 0, 40, 8, -80, -16, 22, 104, -29, 60, -37, -21, 148, 31, 164, 132, 53, -42, 113, 70,
        50, 59, 50, 157, 192, -65, 70, 125, 77, 110, 199, 196, -38, 2, 164, 73, -53, 167, 21, 118,
        129,
    ];

    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 0usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 1usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 2usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 3usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 4usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 5usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 6usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 7usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 8usize));
    assert_eq!(&lhs, where_k_th_ordinal_element_greater(&lhs, &rhs, 9usize));
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 10usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 11usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 12usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 13usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 14usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 15usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 16usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 17usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 18usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 19usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 20usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 21usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 22usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 23usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 24usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 25usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 26usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 27usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 28usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 29usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 30usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 31usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 32usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 33usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 34usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 35usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 36usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 37usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 38usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 39usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 40usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 41usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 42usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 43usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 44usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 45usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 46usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 47usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 48usize)
    );
    assert_eq!(
        &lhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 49usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 99usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 98usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 97usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 96usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 95usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 94usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 93usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 92usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 91usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 90usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 89usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 88usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 87usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 86usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 85usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 84usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 83usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 82usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 81usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 80usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 79usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 78usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 77usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 76usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 75usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 74usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 73usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 72usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 71usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 70usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 69usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 68usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 67usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 66usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 65usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 64usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 63usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 62usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 61usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 60usize)
    );
    assert_eq!(
        &rhs,
        where_k_th_ordinal_element_greater(&lhs, &rhs, 59usize)
    );
}
