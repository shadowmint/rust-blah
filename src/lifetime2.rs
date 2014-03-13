#[feature(macro_rules)];

macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[deriving(Show)]
struct Foo<T> {
  data: ~T
}

impl<T> Foo<T> {

  // Notice how the 'pontless_marker' variable is passed into this function for
  // no reason other than that it allows us to copy the lifetime scope of the 
  // 'marker' variable in the test below, so the lifetime of the returned pointer
  // is valid for that block.
  fn returns_to_scope_with_marker<'a>(&'a self, pointless_marker:&'a int) -> &'a ~T {
    return &self.data;
  }

  // This doesn't work, but it should. You should be able to invoke this function
  // as something like: let marked = bar.returns_to_scope::<LIFETIME???>();
  /*
  fn returns_to_scope<'a>(& self) -> &'a ~T {
    return &self.data;
  }
  */
}

#[test]
fn test_lifetime_return_scope() {
  let bar = Foo { data: ~Foo { data: ~10 } };
  {
    let marker = 10;
    let marked = bar.returns_to_scope_with_marker(&marker);
    trace!("{:?}", marked);
  }
}
