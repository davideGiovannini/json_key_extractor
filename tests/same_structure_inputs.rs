use json_key_extractor::process_element;
use quickcheck::quickcheck;

quickcheck! {
    fn int_values(first: i32, second: i32) -> bool {
        let first = serde_json::from_str(&format!("{}", first)).unwrap();
        let second = serde_json::from_str(&format!("{}", second)).unwrap();
        process_element(first) == process_element(second)
    }

    fn float_values(first: f32, second: f32) -> bool {
        if !(first.is_finite() && second.is_finite()){
            // Skip invalid json values (nan and infinite)
            return true;
        }
        let first = first.into();
        let second = second.into();
        process_element(first) == process_element(second)
    }
    fn bool_values(first: bool, second: bool) -> bool {
        let first = serde_json::from_str(&format!("{}", first)).unwrap();
        let second = serde_json::from_str(&format!("{}", second)).unwrap();
        process_element(first) == process_element(second)
    }
}

#[test]
fn it_works() {
    let input_1 = serde_json::from_str(
        r#"
{
    "key1": 2,
    "array": [2.0, 1.0]
}
"#,
    )
    .unwrap();

    let input_2 = serde_json::from_str(
        r#"
{
    "array": [1.0, 40.1],
    "key1": 45
}
"#,
    )
    .unwrap();

    assert_eq!(process_element(input_1), process_element(input_2));
}
