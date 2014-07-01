use _macros;

#[deriving(PartialEq, Show)]
struct EventOne {
  x: f64,
  y: f64
}

#[deriving(PartialEq, Show)]
struct EventTwo {
  x: int,
  y: int
}

#[deriving(PartialEq, Show)]
enum ComplexEvent {
  A(EventOne, &'static str),
  B(EventTwo, &'static str)
}

#[test]
fn test_lifetime_scope() {
  let x = A(EventOne { x: 0.1, y: 0.1}, "Hello");
  let y = B(EventTwo { x: 1, y: 2}, "Hello2");
  let mut z:&EventOne = &EventOne { x: 0.0, y: 0.0 };

  match x {
    A(ref a, _) => z = a,
    B(b, _) => trace!("{}", b)
  }

  trace!("Accessed values: {}", z);
}
