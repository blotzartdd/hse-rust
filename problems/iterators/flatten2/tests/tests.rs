use flatten2::flat_map;

#[test]
fn empty() {
    assert_eq!(flat_map(std::iter::empty(), |x: Vec<()>| { x }).count(), 0);
}

#[test]
fn chars() {
    assert_eq!(
        flat_map(vec!["a", "b"].into_iter(), |x| { x.chars() }).count(),
        2
    );
}

#[test]
fn strings() {
    assert_eq!(
        flat_map(vec!["al", "bet"].into_iter(), |x| { x.chars() }).count(),
        5
    );
}

#[test]
fn cyrillic() {
    let merged_bytes: Vec<u8> =
        flat_map(vec!["Привет", " ", "мир", "!"].into_iter(), |x| {
            x.bytes()
        })
        .collect();
    let merged_string = String::from_utf8(merged_bytes);
    assert_eq!(merged_string, Ok("Привет мир!".to_string()));
}

#[test]
fn concatenation_test() {
    let words = ["alpha", "beta", "gamma"];
    let merged: String = flat_map(words.iter(), |s| s.chars()).collect();
    assert_eq!(merged, "alphabetagamma");
}

#[test]
fn polymorphism_test() {
    let mut iterators: Vec<Box<dyn Iterator<Item = i64>>> = vec![];
    let source_iterator = vec![1, 2, 3, 4].into_iter();
    for multiplier in 1..11 {
        let iterator = Box::new(
            source_iterator
                .clone()
                .flat_map(move |x| vec![x as i64; multiplier].into_iter()),
        );
        iterators.push(iterator);
    }

    let collection: Vec<i64> = iterators.into_iter().map(|x| x.sum()).collect();
    assert_eq!(collection, (10..101).step_by(10).collect::<Vec<i64>>())
}
