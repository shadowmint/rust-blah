struct Foo {
  value:usize
}

trait HasusizeValue {
  fn as_usize(self) -> usize;
}

impl Foo {
  fn add<T:HasusizeValue>(&mut self, value:T) {
    self.value += value.as_usize();
  }
}

impl HasusizeValue for isize {
  fn as_usize(self) -> usize {
    return self as usize;
  }
}

impl HasusizeValue for f64 {
  fn as_usize(self) -> usize {
    return self as usize;
  }
}

#[test]
fn test_add_with_int()
{
  let mut x = Foo { value: 10 };
  x.add(10isize);
  assert!(x.value == 20);
}

#[test]
fn test_add_with_float()
{
  let mut x = Foo { value: 10 };
  x.add(10.0f64);
  assert!(x.value == 20);
}
