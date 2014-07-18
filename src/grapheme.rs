extern crate unicode;

#[test]
fn test_grapheme() {
  let mut x = "a\u0310eハローワールド".graphemes(true);
  for c in x {
    let y:&str = c;
    trace!("{}", c);
  }
}
