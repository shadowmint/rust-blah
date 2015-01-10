enum FooBar<U,V> {
  Foo(U),
  Bar(V)
}

fn doit(x:FooBar<isize,f32>) -> bool {
  match x {
    FooBar::Foo(x) => { trace!("Got an int"); return false; },
    FooBar::Bar(x) => { trace!("Got a f32"); return true; }
  }
}

#[test]
fn test_overloading_foo() {
  let x = doit(FooBar::Foo(10));
  assert!(!x);
}

#[test]
fn test_overloading_bar() {
  let x = doit(FooBar::Bar(10.0));
  assert!(x);
}
