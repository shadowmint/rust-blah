#[test]
fn find_in_vec() {
  let foo = vec!(5usize, 4, 1, 2, 3);
  trace!("FIND");
  trace!("1: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 1usize }));
  trace!("2: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 2usize }));
  trace!("3: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 3usize }));
  trace!("4: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 4usize }));
  trace!("5: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 5usize }));
}
