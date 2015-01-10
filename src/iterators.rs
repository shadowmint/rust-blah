use std::iter::Iterator;

struct Foo {
  offset:usize,
  x:[isize; 8]
}

impl Iterator for Foo {
  type Item = (* mut Foo, isize);
  fn next(& mut self) -> Option<(* mut Foo, isize)> {
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

impl Iterator for Bar {
  type Item = (* mut Foo, isize);
  fn next(& mut self) -> Option<(* mut Foo, isize)> {
    return self.foo.next();
  }
}

fn test_it() {
  let mut x = Bar { foo: Box::new(Foo { offset: 8, x: [1, 2, 3, 4, 5, 6, 7, 8] }) };
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
