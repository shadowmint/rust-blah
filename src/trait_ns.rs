use _macros;

mod foo {
  pub trait Foo {
    fn hello(&self);
  }
}

mod bar {
  pub trait Foo {
    fn hello(&self, x:int);
  }
}

struct Item;

impl foo::Foo for Item {
  fn hello(&self) {
    trace!("Hello for foo");
  }
}

impl bar::Foo for Item {
  fn hello(&self, x:int) {
    trace!("Hello for bar: {}", x);
  }
}

mod p1 {
  use super::foo::Foo; // Clobbered by next use
  use super::bar::Foo;
  use super::Item;

  #[test]
  fn test_can_use_foo_hello() {
    let x = Item;
    x.hello(1);
  }
}

mod p2 {
  use super::bar::Foo; // Clobbered by next use
  use super::foo::Foo;
  use super::Item;

  #[test]
  fn test_can_use_foo_hello() {
    let x = Item;
    x.hello();
  }
}
