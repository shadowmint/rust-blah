use _macros;

#[deriving(Show)]
struct Foo {
  value: int
}

#[deriving(Show)]
struct Bar {
  data: Option<Box<Foo>>
}

#[deriving(Show)]
enum BarErr {
  Nope
}

impl Bar {
  fn borrow<'a>(&'a mut self) -> Result<&'a Box<Foo>, BarErr> {
    match self.data {
      Some(ref e) => return Ok(e),
      None => return Err(BarErr::Nope)
    }
  }
}

#[test]
fn test_create_indirect() {
  let y = box Foo { value: 10 };
  let mut x = Bar { data: Some(y) };
  let mut x2 = Bar { data: None };
  {
    match x.borrow() {
      Ok(ref foo) => trace!("Found {}", foo.value),
      Err(Nope) => trace!("Bleh")
    }
  }
  {
    let z = x2.borrow();
    trace!("Z: {}", z);
  }
}
