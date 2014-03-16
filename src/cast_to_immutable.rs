use _macros;

fn doit(x: &int) -> int {
  return *x + 10;
}

#[test]
fn test_cast_to_immutable() {
   let x = 10;
   let y = &10;
   let z = doit(y);
   trace!("{:?}", z);
}
