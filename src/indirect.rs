#[derive(Debug)]
struct Indirect<T> {
  _data: T
}

impl<T> Indirect<T> {

  // Create a new generic item
  fn new(t:T) -> Indirect<T> {
    return Indirect { _data: t };
  }

  // Open a mutable reference to T
  fn get<'a>(&'a mut self) -> *const T {
    return &self._data as *const T;
  }
}

#[test]
fn test_create_indirect() {
  let mut x = Indirect::new(Box::new(10usize));
  unsafe {
    let y = x.get();
    let z = x.get();
    trace!("{:?}", *y);
    trace!("{:?}", *z);
  }
}
