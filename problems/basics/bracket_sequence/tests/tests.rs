use bracket_sequence::is_correct_bracket_sequence;

#[test]
fn it_works() {
    // Basic cases
    assert!(is_correct_bracket_sequence("()"));
    assert!(is_correct_bracket_sequence("()[]{}"));
    assert!(!is_correct_bracket_sequence("(]"));
    assert!(!is_correct_bracket_sequence("([)]"));

    // Edge cases
    assert!(is_correct_bracket_sequence("")); // An empty string is considered valid.
    assert!(!is_correct_bracket_sequence("(")); // Single unmatched bracket.
    assert!(!is_correct_bracket_sequence(")")); // Single unmatched bracket.
    assert!(is_correct_bracket_sequence("[{()}]")); // Nested brackets.

    // Longer sequences
    assert!(is_correct_bracket_sequence("[[][]{}{{}}()]")); // Multiple valid sequences combined.
    assert!(!is_correct_bracket_sequence("[[][]{}{{}}(])")); // Combined sequences with a single mismatch.

    // Mismatch at start or end
    assert!(!is_correct_bracket_sequence("(([]{}))}")); // Extra bracket at the end.
    assert!(!is_correct_bracket_sequence("{(([]{}))")); // Extra bracket at the start.

    // Incorrect bracket types
    assert!(!is_correct_bracket_sequence("{(})")); // Incorrect closing bracket type.
}
