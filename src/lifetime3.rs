use _macros;

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
