#[test]
fn test_vectors() {
  let x = [1isize, 2isize, 3isize];
  for y in ["1", "2", "3"].iter() {
    trace!("vector: {:?}", y);
  }
  for i in x.iter() {
    trace!("vector: {:?}", i);
  }
}
