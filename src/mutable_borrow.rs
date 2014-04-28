macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[deriving(Show)] 
struct Foo {
  value: int
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

#[deriving(Show)] 
struct Bar {
  data: Option<~Foo>
}

#[deriving(Show)] 
enum BarErr {
  Nope
}

impl Bar {
  fn borrow<'a>(&'a mut self) -> Result<&'a mut ~Foo, BarErr> {
    match self.data {
      Some(ref mut e) => return Ok(e),
      None => return Err(Nope)
    }
  }

  fn return_self<'a>(&'a mut self) -> &'a mut Bar {
    return self;
  }

  fn return_foo<'a>(&'a mut self) -> Result<&'a mut ~Foo, BarErr> {
    match self.data {
      Some(ref mut x) => return Ok(x),
      None => return Err(Nope)
    }
  }

  fn checker(&self) {
    trace!("Hello Checker\n");
  }
}

#[test]
fn test_recurse_foo() {
  let y = ~Foo { value: 10 };
  let mut x = Bar { data: Some(y) };
  match x.return_self().return_self().return_foo() {
    Ok(foo) => foo.checker(),
    _ => {}
  }
}

#[test]
fn test_recurse_self() {
  let y = ~Foo { value: 10 };
  let mut x = Bar { data: Some(y) };
  x.return_self().return_self().return_self().checker();
}

#[test]
fn test_create_indirect() {
  let y = ~Foo { value: 10 };
  let mut x = Bar { data: Some(y) };
  let mut x2 = Bar { data: None };
  { 
    match x.borrow() {
      Ok(foo) => { foo.inc(); trace!("Found {:?}", foo.value); },
      Err(Nope) => trace!("Bleh")
    }
  }
  { 
    let z = x2.borrow();
    trace!("Z: {:?}", z);
  }
}