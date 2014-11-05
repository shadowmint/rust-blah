use _macros;

#[deriving(Show)]
struct Foo<T> {
  data: Box<T>
}

impl<T> Foo<T> {

  // Notice how the 'pontless_marker' variable is passed into this function for
  // no reason other than that it allows us to copy the lifetime scope of the
  // 'marker' variable in the test below, so the lifetime of the returned pointer
  // is valid for that block.
  fn returns_to_scope_with_marker<'a>(&'a self, pointless_marker:&'a int) -> &'a Box<T> {
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
  let bar = Foo { data: box Foo { data: box 10i } };
  {
    let marker = 10i;
    let marked = bar.returns_to_scope_with_marker(&marker);
    trace!("{}", marked);
  }
}

static mut VALUE:int = 10;

unsafe fn returns_value() -> &'static int {
  return &VALUE;
}

#[test]
fn test_return_a_pointer() {
  let x:&int;
  {
    unsafe { x = returns_value(); }
    trace!("Hi: {}", x);
  }
  let y = x;
  trace!("Hi2: {}", y);
}
