use split::split;

#[test]
fn simple_test() {
    assert_eq!(
        vec!["Hello", "world"],
        split("Hello world", " ").collect::<Vec<&str>>()
    );
}

#[test]
fn path_test() {
    fn check_path(some_path: &str) {
        assert_eq!(
            some_path.split("/").collect::<Vec<&str>>(),
            split(some_path, "/").collect::<Vec<&str>>()
        );
        let delimiter = if some_path.starts_with("/") {
            some_path.chars().nth(0).unwrap().to_string()
        } else {
            "/".to_string()
        };
        assert_eq!(
            some_path,
            split(some_path, &delimiter)
                .collect::<Vec<&str>>()
                .join("/")
        );
    }
    check_path("/road/to/hell");
    check_path("C:/Путь/до/директорией/с/лучшей/домашкой/на/свете");
    check_path("C://Windows/Linux/Mac/Альт");
    check_path("tools/rover/gold_solution");
}

#[test]
fn smart() {
    fn check(input: &str, delimiter: &str) {
        if delimiter == "" {
            assert_eq!(
                split(input, delimiter).take(100).collect::<Vec<&str>>(),
                input
                    .split(delimiter)
                    .take(100)
                    .filter(|&x| x != "")
                    .collect::<Vec<&str>>()
            );
        } else {
            assert_eq!(
                split(input, delimiter).take(100).collect::<Vec<&str>>(),
                input.split(delimiter).take(100).collect::<Vec<&str>>()
            );
        }
    }

    check("abracadabra", "a");
    check("abracadabra", "ab");
    check("abracadabra", "abra");
    check("abracadbara", "");
    check("абракадабра", "");
    check("абракадабра", "аб");
    check("abracadbara", "аб");
    check("\"abracadbara\"", "\"");
    check("a❤b❤c❤", "❤");
}
