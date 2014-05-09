use _macros;

#[deriving(Show)] 
struct Indirect<T> {
  _data: T
}

impl<T> Indirect<T> {

  // Create a new generic item
  fn new(t:T) -> Indirect<T> {
    return Indirect { _data: t };
  }

  // Open a mutable reference to T
  fn get<'a>(&'a mut self) -> *T {
    return &self._data as *T;
  }
}

#[test]
fn test_create_indirect() {
  let mut x = Indirect::new(box 10);
  unsafe {
    let y = x.get();
    let z = x.get();
    trace!("{}", *y);
    trace!("{}", *z);
  }
}
