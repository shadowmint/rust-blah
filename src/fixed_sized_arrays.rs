fn return_array() -> [u8; 2] {
  return [0u8, 1u8];
}

#[test]
fn test_fixed_size() {
  let x:[u8; 2] = [0u8, 1u8];
  assert!(x[0] == 0u8);
  assert!(x[1] == 1u8);
}

#[test]
fn test_fixed_size_return() {
  let x = return_array();
  assert!(x[0] == 0u8);
  assert!(x[1] == 1u8);
}
