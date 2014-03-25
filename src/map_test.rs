use _macros;

#[test]
fn test_map_to_new_values() {
  let mut x = ~[1, 2, 3];
  let mut output = x.iter().map(|y| {
    trace!("Value: {}", y);
    return *y + 1;
  });
  for x in output {
    trace!("{}", x);
  }
}

#[test]
fn test_map_replacing_values() {
  let mut x = ~[1, 2, 3];
  let z = x.mut_iter().map(|y| {
    return *y + 1;
  });
}
