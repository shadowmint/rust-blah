macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

pub fn do_something() -> uint {
  return 99;
}

struct Blah<'a> {
  x:&'a fn() -> uint
}

#[test]
fn test_this_thing() {
  let x = Blah { x: &do_something };
  let y = (*x.x)();
  trace!("{}", y);
}
