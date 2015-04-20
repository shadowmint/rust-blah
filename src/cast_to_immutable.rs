fn doit(x: &isize) -> isize {
  return *x + 10;
}

#[test]
fn test_cast_to_immutable() {
   let x = 10isize;
   let y = &10isize;
   let z = doit(y);
   trace!("{:?}", z);
}
