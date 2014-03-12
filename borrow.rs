macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[deriving(Show)] 
struct Foo {
  value: int
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
  fn borrow<'a>(&'a mut self) -> Result<&'a ~Foo, BarErr> {
    match self.data {
      Some(ref e) => return Ok(e),
      None => return Err(Nope)
    }
  }
}

#[test]
fn test_create_indirect() {
  let y = ~Foo { value: 10 };
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