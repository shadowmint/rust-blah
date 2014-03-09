macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[deriving(Show)] 
struct Foo;

#[deriving(Show)] 
struct Bar {
  data: Option<~Foo>
}

#[deriving(Show)] 
enum BarErr {
  Nope
}

impl Bar {
  fn borrow<'a>(&mut self) -> Result<&~Foo, BarErr> {
    match self.data {
      Some(e) => return Ok(&e),
      None => return Err(Nope)
    }
  }
}

#[test]
fn test_create_indirect() {
  let mut x = Bar { data: Some(~Foo) };
  let mut x2 = Bar { data: None };
  { 
    let y = x.borrow();
    trace!("{}", y);
  }
  { 
    let z = x.borrow();
    trace!("{}", z);
  }
}
