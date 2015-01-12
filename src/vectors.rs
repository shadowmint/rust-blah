#[test]
fn test_vectors() {
  let x = [1is, 2is, 3is];
  for y in ["1", "2", "3"].iter() {
    trace!("vector: {:?}", y);
  }
  for i in x.iter() {
    trace!("vector: {:?}", i);
  }
}
