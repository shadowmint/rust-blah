use _macros;

#[test]
fn test_vectors() {
  let x = [1, 2, 3];
  for y in ["1", "2", "3"].iter() {
    trace!("vector: {:?}", y);
  }
  trace!("vector: {:?}", x);
}
