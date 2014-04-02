macro_rules! trace(
  ($($arg:tt)*) => (
    { ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); }
  );
)

#[test]
fn test_vectors() {
  let x = [1, 2, 3];
  for y in ["1", "2", "3"].iter() {
    trace!("vector: {:?}", y);
  }
  trace!("vector: {:?}", x);
}
