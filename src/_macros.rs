#![macro_escape]

#[macro_export]
macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

#[test]
fn test_trace_macro() {
  trace!("Hello World {} {}", 1, 2);
}
