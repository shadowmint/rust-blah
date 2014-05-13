extern crate libc;

use self::libc::c_void;
use std::ops::Drop;
use std::intrinsics::forget;
use std::mem::transmute;

struct Foo { x: int }

impl Drop for Foo {
  fn drop(&mut self) {
    println!("We discarded our foo thanks!");
  }
}

fn main() {
  let mut x = box Foo { x: 10 };

  let mut y = & mut *x as * mut Foo;
  println!("Address Y: {:x}", y as uint);

  let mut z = y as * mut c_void;
  println!("Address Z: {:x}", z as uint);

  // Forget x so we don't worry about it
  unsafe { forget(x); }

  {
    let res_x:Box<Foo>;
    unsafe {
      let mut res_z = z as * mut Foo;
      println!("Ressurected Z: {:x}", z as uint);

      let mut res_y = & mut (*res_z);
      println!("Ressurected Y: {:x}", y as uint);

      let mut tmp:Box<Foo> = transmute(res_y);
      println!("The things~");
    }
    println!("Outside space");
  }
}
