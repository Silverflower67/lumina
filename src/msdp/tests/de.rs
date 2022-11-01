#[test]
pub(crate) fn test_struct() {
    use super::from_slice;
    use serde::Deserialize;
   #[derive(Deserialize, PartialEq, Debug)]
   struct Amogus {
    sus: bool,
    id: i32
   }
   let input = &b"\x03\x01SUS\x02TRUE\x01ID\x0269\x04"[..];
   let value: Amogus = from_slice(input).expect("Failed deserialization");
   let expected = Amogus {sus: true, id: 69};
   assert_eq!(value,expected)
}
