#[test]
pub(crate) fn test_struct() {
    use super::to_vec;
    use serde::{Serialize};
    #[derive(Serialize)]
    struct User {
        username: String,
        tag: i32
    }
    let test = User {username: "Silverflower".to_string(), tag: 8414};
    let expected = &b"\x03\x01USERNAME\x02Silverflower\x01TAG\x028414\x04"[..];
    assert_eq!(to_vec(&test).unwrap(),expected.to_vec())
}
