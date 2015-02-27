use std::any::TypeId;

trait Foo : ::std::marker::MarkerTrait {
  type Item: 'static;
  fn foo() -> isize {
    return 1;
  }
}

struct Bar;
struct Foo1;

impl Foo for Foo1 {
  type Item = Bar;
}

impl Foo1 {
  fn s<T: 'static>(&self) -> bool where Self: Foo {
    let rtn = TypeId::of::<<Self as Foo>::Item>() == TypeId::of::<T>();
    return rtn;
  }
}

#[test]
fn test_methods() {
  trace!("Hello {:?}!", <Foo1 as Foo>::foo());

  let f = Foo1;
  trace!("World {:?}!", f.s::<Foo1>());
  trace!("World {:?}!", f.s::<Bar>());
}
