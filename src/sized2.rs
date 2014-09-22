use std::iter::Iterator;
use std::default::Default;
use std::mem::transmute;

// Helpers
trait Iter<T> for Sized? { 
  fn next(&mut self) -> Option<T>; 
}

struct HasIter<Sized? T: Iter<int> + 'static> { 
  t: T 
}

impl<T: Iter<int>> Iterator<int> for HasIter<T> { 
    fn next(&mut self) -> Option<int> {
        return self.t.next();
    }
}

// Bar setup
struct Bar { v: int }
impl Iter<int> for Bar {
    fn next(&mut self) -> Option<int> {
        self.v += 1;
        if self.v < 10 {
          return Some(self.v);
        }
        return None;
    }
}
impl Default for Bar {
  fn default() -> Bar {
    return Bar { v: 0 };
  }
}

// Foo setup
struct Foo { v: int }
impl Iter<int> for Foo {
    fn next(&mut self) -> Option<int> {
        self.v += 1;
        if self.v < 20 {
          return Some(self.v);
        }
        return None;
    }
}
impl Default for Foo {
  fn default() -> Foo {
    return Foo { v: 0 };
  }
}

struct FooBar;

// Factory for any T
fn iter<T: Default + Iter<int>> () -> HasIter<T> {
  unsafe {
    let foo:T = Default::default();
    return HasIter { t: foo };
  }
}

#[test]
fn test_sized() {
  let mut total = 0i;
  for i in iter::<Bar>() { total += i; }
  assert!(total == 45);

  let mut total = 0i;
  for i in iter::<Foo>() { total += i; }
  assert!(total == 190);

  // Won't work, doesn't implement Iter
  // let mut total = 0i;
  // for i in iter::<FooBar>() { total += i; }
  // assert!(total == 190);
}
