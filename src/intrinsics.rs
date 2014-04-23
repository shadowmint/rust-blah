extern crate libc;

use std::intrinsics::size_of;
use self::libc::malloc;
use self::libc::free;
use self::libc::c_void;

macro_rules! trace(
  ($($arg:tt)*) => (
    { let _ = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); }
  );
)

#[deriving(Show)]
struct Foo {
  x: int,
  y: *int
}

#[test]
fn test_intrinsic_memory_block() {
  let x = 0;
  let mut p = Foo { x: 10, y: &x as *int};
  let mut q:* mut Foo;
  unsafe {
    trace!("Block size: {}", size_of::<Foo>());
    q = & mut p as * mut Foo;
    trace!("Foo... {}", *q);
  }

  // Create new memory block
  unsafe {
    let data = malloc(size_of::<Foo>() as u64);
    trace!("alloc: {}", data as u64);
    q = data as * mut Foo;
    trace!("End block");
  }
  trace!("Exit scope");

  // Yay, we have a non-memory managed block!
  unsafe {
    trace!("scope1");
    (*q).x = 100;
    trace!("scope3");
    (*q).y = &x as *int;
    trace!("Hii... {}", *q);
    trace!("scope2");
  }
  trace!("WTF");

  // Destory memory block
  unsafe {
    let data = q as * mut c_void;
    trace!("free: {}", data as u64);
    free(data);
  }
}

#[test] 
fn test_forget() {
}
