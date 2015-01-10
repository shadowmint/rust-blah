macro_rules! nstrcmp(
  ($x:ident, $y:ident, $n:expr) => (
    $x.len() >= $n && $y.len() >= $n && $x.slice(0, $n) == $y.slice(0, $n)
  );
);

#[test]
fn test_nstrcmp() {
  let x = "Hello World";
  let y = "Hello";
  let z = "hello";
  trace!("nstrcmp: {}", nstrcmp!(x, y, 5));
  trace!("nstrcmp: {}", nstrcmp!(x, y, 10));
  trace!("nstrcmp: {}", nstrcmp!(x, y, 3));
  trace!("nstrcmp: {}", nstrcmp!(z, y, 4));
}
