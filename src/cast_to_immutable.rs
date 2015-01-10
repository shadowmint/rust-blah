fn doit(x: &isize) -> isize {
  return *x + 10;
}

#[test]
fn test_cast_to_immutable() {
   let x = 10is;
   let y = &10is;
   let z = doit(y);
   trace!("{}", z);
}
