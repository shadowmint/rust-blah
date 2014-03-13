use std::default::Default;
use _macros;

struct CItem {
  x: int,
  y: int,
  _z: ~str
}

// Special default trait for default values
impl Default for CItem {
  fn default() -> CItem {
    return CItem { x: -1, y: 0, _z: ~"None" };
  }
}

// NB. See how we use the defaults
impl CItem {
  fn new(name:~str) -> CItem {
    let rtn = CItem { _z: name, ..Default::default() };
    return rtn;
  }
}

// A trait (ie. interface) we have on CItem
trait HasFun {
  fn hello(&mut self);
}

// ...and the trait impl
impl HasFun for CItem {
  fn hello(&mut self) {
    trace!("{2}: {0}: {1}\n", self.x, self.y, self._z);
    self.x += 1;
  }
}

#[test]
fn test_can_create_struct() {
  let mut value = CItem::new(~"Struct Tests");
  value.hello();
  value.hello();
  value.hello();
}
