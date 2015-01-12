#[test]
fn find_in_vec() {
  let foo = vec!(5us, 4, 1, 2, 3);
  trace!("FIND");
  trace!("1: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 1us }));
  trace!("2: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 2us }));
  trace!("3: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 3us }));
  trace!("4: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 4us }));
  trace!("5: {:?}", foo.iter().enumerate().find(|r| { return *(*r).1 == 5us }));
}
