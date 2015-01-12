extern crate unicode;

#[test]
fn test_grapheme() {
  let mut x = "aハローワールド".graphemes(true);
  for c in x {
    let y:&str = c;
    trace!("{:?}", c);
  }
}
