trait Foo {
  fn foo() -> isize {
    return 1;
  }
}

struct Foo1;

impl Foo for Foo1 {}

#[test]
fn test_methods() {
  println!("Hello {:?}!", <Foo1 as Foo>::foo());
}
