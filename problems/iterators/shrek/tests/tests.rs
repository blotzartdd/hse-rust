#[cfg(test)]
mod test_count_words {
    use shrek::count_lines_with_word;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn open() -> impl Iterator<Item = String> {
        let file = File::open("shrek.txt").expect("Something wrong");
        let reader = BufReader::new(file);
        reader.lines().map(|s| s.unwrap())
    }

    #[test]
    fn test_shrek() {
        assert_eq!(count_lines_with_word(open(), "shrek"), 62);

        let song = ["somebody", "once", "told", "me"];
        let count: Vec<usize> = song
            .iter()
            .map(|word| count_lines_with_word(open(), word))
            .collect();
        assert_eq!(count, [6, 3, 5, 280]);
    }
}

#[cfg(test)]
mod test_top_k_longest {
    use shrek::top_k_longest;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn open() -> impl Iterator<Item = String> {
        let file = File::open("shrek.txt").expect("Something wrong");
        let reader = BufReader::new(file);
        reader.lines().map(|s| s.unwrap())
    }

    #[test]
    fn test_lines() {
        assert_eq!(
            top_k_longest(open(), 5)
                .map(|(_, line)| line)
                .collect::<Vec<String>>(),
            vec![
                "-Hi, everyone. Havin' a good time, are ya? I love DuLoc, first at all. # The longest line",
                "-Then you got to, got to try a little tenderness. The chicks love that # second longest",
                "that? Shrek, wait, wait! Wait a minute! You wanna do this right, don't # little shrter",
                "All right, hop on and hold on tight. I haven't had a chance to install # even smaller",
                "-All right, all right.Don't get all slobbery. No one likes a kiss ass. # fifth lngst"
            ],
        )
    }

    #[test]
    fn test_index() {
        assert_eq!(
            top_k_longest(open(), 5)
                .map(|(size, _)| size)
                .collect::<Vec<usize>>(),
            vec![2871, 2835, 2809, 2785, 2783],
        )
    }
}

#[cfg(test)]
mod test_words {
    use shrek::words_counter_iter;
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    fn open() -> impl Iterator<Item = String> {
        let file = File::open("shrek.txt").expect("Something wrong");
        let reader = BufReader::new(file);
        reader.lines().map(|s| s.unwrap())
    }

    #[test]
    fn test_words() {
        assert_eq!(words_counter_iter(open(), "?").count(), 253);
        assert_eq!(
            words_counter_iter(open(), "?")
                .filter(|(count, _)| count > &(1 as usize))
                .count(),
            26
        );

        assert_eq!(words_counter_iter(open(), "laughing").count(), 8);
        assert!(words_counter_iter(open(), "laughing").all(|(size, _)| size == 1));
    }
}
