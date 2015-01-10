struct Foo;

enum FooArgs {

}

impl Foo {
  fn add2(&self, a:isize, b:isize) -> isize {
    return a + b;
  }
  fn add3(&self, a:isize, b:isize, c:isize) -> isize {
    return a + b + c;
  }
  fn add4(&self, a:isize, b:isize, c:isize, d:isize) -> isize {
    return a + b + c + d;
  }
}

macro_rules! add(
  ($foo:ident, $a:expr, $b:expr) => (
    $foo.add2($a, $b);
  );
  ($foo:ident, $a:expr, $b:expr, $c:expr) => (
    $foo.add3($a, $b, $c);
  );
  ($foo:ident, $a:expr, $b:expr, $c:expr, $d:expr) => (
    $foo.add4($a, $b, $c, $d);
  );
)

#[test]
fn test_var_args() {
  let foo = Foo;
  println!("Add:{}", add!(foo, 1, 2));
  println!("Add:{}", add!(foo, 1, 2, 3));
  println!("Add:{}", add!(foo, 1, 2, 3, 4));
}
