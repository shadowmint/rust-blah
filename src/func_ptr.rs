pub fn do_something() -> usize {
  return 99;
}

struct Blah<'a> {
  x:&'a (Fn<(),usize> + 'a)
}

#[test]
fn test_this_thing() {
  let x = Blah { x: &do_something };
  let y = (*x.x).call(());
  trace!("{}", y);
}
