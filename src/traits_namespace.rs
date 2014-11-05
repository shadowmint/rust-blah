use std::default::Default;
use _macros;

struct CItem {
  x: int
}

trait CTItem {
  fn x(&self) -> int;
}

impl CTItem for CItem {
  fn x(&self) -> int {
    return self.x;
  }
}

#[test]
fn test_can_create_struct() {
  let mut value = CItem { x: 10 };
  trace!("?? {}", value.x);
  trace!("?? {}", value.x());
}
