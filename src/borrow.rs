#[derive(Debug)]
struct Foo {
  value: isize
}

#[derive(Debug)]
struct Bar {
  data: Option<Box<Foo>>
}

#[derive(Debug)]
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
  let y = Box::new(Foo { value: 10 });
  let mut x = Bar { data: Some(y) };
  let mut x2 = Bar { data: None };
  {
    match x.borrow() {
      Ok(ref foo) => trace!("Found {:?}", foo.value),
      Err(_) => trace!("Bleh")
    }
  }
  {
    let _ = x2.borrow();
    // trace!("Z: {:?}", z);
  }
}
