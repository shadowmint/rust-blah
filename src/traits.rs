use std::default::Default;

struct CItem {
  x: isize,
  y: isize,
  _z: &'static str
}

// Special default trait for default values
impl Default for CItem {
  fn default() -> CItem {
    return CItem { x: -1, y: 0, _z: "None" };
  }
}

// Helper macro creates things!
macro_rules! default(
  ($T:ident, $($k:ident: $v:expr), *) => (
    $T { $($k: $v), *, ..Default::default() }
  );
);

#[test]
fn test_trace_macro() {
  trace!("Hello World {:?} {:?}", 1isize, 2isize);
}

// NB. See how we use the defaults
impl CItem {
  fn new(name:&'static str) -> CItem {
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
  let mut value = default!(CItem, _z: "Struct Tests", x: 10);
  value.hello();
  value.hello();
  value.hello();
}
