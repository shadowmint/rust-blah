#[macro_escape];

use std::io::stdout;
use std::fmt;

#[macro_export]
macro_rules! trace(
  ($($arg:tt)*) => (
    ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*))
  );
)

#[test]
fn test_trace_macro() {
  trace!("Hello World {} {}", 1, 2);
}
