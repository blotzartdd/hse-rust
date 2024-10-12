mod add {
    use basics::add2;

    #[test]
    fn numbers() {
        let it = add2(1..);
        let v: Vec<i32> = it.take(10).collect();
        assert_eq!(v, vec![3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn vector() {
        let it = add2(vec![10, 20, 30, 40].into_iter());
        let v: Vec<i32> = it.take(4).collect();
        assert_eq!(v, vec![12, 22, 32, 42]);
    }
}

mod div3 {
    use basics::div3;

    #[test]
    fn it_works() {
        let it = div3();
        let v: Vec<i32> = it.take(10).collect();
        assert_eq!(v, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]);
    }
}

mod take_n {
    use basics::take_n;

    #[test]
    fn it_works() {
        let v = take_n(1.., 5);
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn it_works2() {
        let v = take_n(vec![10, 20, 30, 40, 50, 60].into_iter(), 5);
        assert_eq!(v, vec![10, 20, 30, 40, 50]);
    }
}
