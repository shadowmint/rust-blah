use std::default::Default;

struct CItem {
  x: isize
}

trait CTItem {
  fn x(&self) -> isize;
}

impl CTItem for CItem {
  fn x(&self) -> isize {
    return self.x;
  }
}

#[test]
fn test_can_create_struct() {
  let mut value = CItem { x: 10 };
  trace!("?? {:?}", value.x);
  trace!("?? {:?}", value.x());
}
