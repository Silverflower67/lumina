use serde::Serialize;
use super::ser::to_vec;
#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct User {
        username: String,
        tag: i32
    }
    let test = User {username: "Silverflower".to_string(), tag: 8414};
    let expected = &b"\x03\x01username\x02Silverflower\x01tag\x028414\x04"[..];
    assert_eq!(to_vec(&test).unwrap(),expected.to_vec())
}