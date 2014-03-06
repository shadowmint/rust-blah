use _macros;

#[deriving(Eq, Show)]
struct EventOne {
  x: f64,
  y: f64
}

#[deriving(Eq, Show)]
struct EventTwo {
  x: int,
  y: int
}

#[deriving(Eq, Show)]
enum ComplexEvent {
  A(EventOne, ~str),
  B(EventTwo, ~str)
}

#[test]
fn test_lifetime_scope() {
  let x = A(EventOne { x: 0.1, y: 0.1}, ~"Hello");
  let y = B(EventTwo { x: 1, y: 2}, ~"Hello2");
  let mut z:ComplexEvent = x;
  trace!("{}", z);
  z = y;
  trace!("{}", z);
}
