#![macro_escape]

#[macro_export]
macro_rules! trace(
  ($($arg:tt)*) => (
    { 
      let tmp = format_args!(::std::fmt::format, $($arg)*);
      let value:&str = tmp.as_slice();
      let x = ::std::io::stdout().write_line(value);
      println!("{}", x); 
    }
  );
)

#[test]
fn test_trace_macro() {
  trace!("Hello World {} {}", 1, 2);
}
