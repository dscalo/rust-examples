use testing;

#[test]
fn test_greeting() {
    assert_eq!(testing::greet("Pete"), "Hey there, Pete");
}