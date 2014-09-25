#[test]
fn find_in_vec() {
  let foo = vec!(5u, 4, 1, 2, 3); 
  trace!("FIND");
  trace!("1: {}", foo.iter().enumerate().find(|r| { return *(*r).1 == 1u }));
  trace!("2: {}", foo.iter().enumerate().find(|r| { return *(*r).1 == 2u }));
  trace!("3: {}", foo.iter().enumerate().find(|r| { return *(*r).1 == 3u }));
  trace!("4: {}", foo.iter().enumerate().find(|r| { return *(*r).1 == 4u }));
  trace!("5: {}", foo.iter().enumerate().find(|r| { return *(*r).1 == 5u }));
}
