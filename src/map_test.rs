#[test]
fn test_map_to_new_values() {
  let mut x = Vec::<isize>::new();
  for i in 0..4 { x.push(i); }
  let mut output = x.iter().map(|y| {
    trace!("Value: {:?}", y);
    return *y + 1;
  });
  for x in output {
    trace!("{:?}", x);
  }
}

#[test]
fn test_map_replacing_values() {
  let mut x = Vec::<isize>::new();
  for i in 0..4 { x.push(i); }
  let z = x.iter().map(|y:&isize| {
    return *y + 1;
  });
}
