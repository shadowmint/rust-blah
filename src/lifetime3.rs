#![feature(macro_rules)]

macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[deriving(Show)]
struct Foo<'a, T> {
  data: &'a T
}

// See how we steal the lifetime of the Foo instance for
// the return value of the function! So cool!
impl<'a, T> Foo<'a, T> {
  fn returns_to_scope(&'a self) -> &'a T {
    self.data
  }
}

#[test]
fn test_scope() {
  let bar = Foo { data: &Foo { data: &10 } };
  let marked;
  {
    marked = bar.returns_to_scope();
    trace!("First: {:?}", marked);
  }
  trace!("Second: {:?}", marked);
}
