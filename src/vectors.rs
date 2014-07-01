use _macros;

#[test]
fn test_vectors() {
  let x = [1i, 2i, 3i];
  for y in ["1", "2", "3"].iter() {
    trace!("vector: {}", y);
  }
  for i in x.iter() {
    trace!("vector: {}", i);
  }
}
