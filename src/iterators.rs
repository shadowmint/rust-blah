use std::iter::Iterator;

struct Foo {
  offset:uint,
  x:[int, ..8]
}

impl Iterator<(* mut Foo, int)> for Foo {
  fn next(& mut self) -> Option<(* mut Foo, int)> {
    if self.offset > 0 {
      self.offset -= 1;
      return Some((self as * mut Foo, self.x[self.offset]));
    }
    return None;
  }
}

struct Bar {
  foo: Box<Foo>
}

impl Iterator<(* mut Foo, int)> for Bar {
  fn next(& mut self) -> Option<(* mut Foo, int)> {
    return self.foo.next();
  }
}

fn main() {
  let mut x = Bar { foo: box Foo { offset: 8, x: [1, 2, 3, 4, 5, 6, 7, 8] } };
  println!("Start");
  for (y, z) in x {
    let q:& mut Foo;
    unsafe {
      q = & mut (*y);
    }
    println!("{} -> {}", q.offset, z);
  }
  println!("End");
}
