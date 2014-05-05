enum FooBar<U,V> {
  Foo(U),
  Bar(V)
}

fn doit(x:FooBar<int,f32>) -> bool {
  match x {
    Foo(x) => { trace!("Got an int"); return false; },
    Bar(x) => { trace!("Got a f32"); return true; }
  }
}

#[test]
fn test_overloading_foo() {
  let x = doit(Foo(10));
  assert!(!x);
}

#[test]
fn test_overloading_bar() {
  let x = doit(Bar(10.0));
  assert!(x);
}
