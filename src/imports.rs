// Notice how we use self:: and use here to import into the local scope
// so that hello is available.
use self::imported::Imported;

mod imported {
  pub trait Imported {
    fn hello(&self, x:isize) -> isize;
  }
}

struct Hi;

impl imported::Imported for Hi {
  fn hello(&self, x:isize) -> isize {
    return x;
  }
}

#[test]
fn test_thing() {
  let value = Hi;
  trace!("{:?}", value.hello(10));
}
