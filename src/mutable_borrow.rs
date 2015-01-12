#[derive(Show)]
struct Foo {
  value: isize
}

impl Foo {
  fn inc(&mut self) {
    self.value += 1;
  }

  fn checker(&mut self) {
    self.value += 1;
    trace!("Hello Foo Checker\n");
  }
}

#[derive(Show)]
struct Bar {
  data: Option<Box<Foo>>
}

#[derive(Show)]
enum BarErr {
  Nope
}

impl Bar {
  fn borrow<'a>(&'a mut self) -> Result<&'a mut Box<Foo>, BarErr> {
    match self.data {
      Some(ref mut e) => return Ok(e),
      None => return Err(BarErr::Nope)
    }
  }

  fn return_self<'a>(&'a mut self) -> &'a mut Bar {
    return self;
  }

  fn return_foo<'a>(&'a mut self) -> Result<&'a mut Box<Foo>, BarErr> {
    match self.data {
      Some(ref mut x) => return Ok(x),
      None => return Err(BarErr::Nope)
    }
  }

  fn checker(&self) {
    trace!("Hello Checker\n");
  }
}

#[test]
fn test_recurse_foo() {
  let y = Box::new(Foo { value: 10 });
  let mut x = Bar { data: Some(y) };
  match x.return_self().return_self().return_foo() {
    Ok(foo) => foo.checker(),
    _ => {}
  }
}

#[test]
fn test_recurse_self() {
  let y = Box::new(Foo { value: 10 });
  let mut x = Bar { data: Some(y) };
  x.return_self().return_self().return_self().checker();
}

#[test]
fn test_create_indirect() {
  let y = Box::new(Foo { value: 10 });
  let mut x = Bar { data: Some(y) };
  let mut x2 = Bar { data: None };
  {
    match x.borrow() {
      Ok(foo) => { foo.inc(); trace!("Found {:?}", foo.value); },
      Err(BarErr::Nope) => trace!("Bleh")
    }
  }
  {
    let z = x2.borrow();
    trace!("Z: {:?}", z);
  }
}
