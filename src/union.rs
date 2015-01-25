#[derive(PartialEq, Debug)]
struct EventOne {
  x: f64,
  y: f64
}

#[derive(PartialEq, Debug)]
struct EventTwo {
  x: isize,
  y: isize
}

#[derive(PartialEq, Debug)]
enum ComplexEvent {
  A(EventOne, &'static str),
  B(EventTwo, &'static str)
}

#[test]
fn test_lifetime_scope() {
  let x = ComplexEvent::A(EventOne { x: 0.1, y: 0.1}, "Hello");
  let y = ComplexEvent::B(EventTwo { x: 1, y: 2}, "Hello2");
  let mut z:&EventOne = &EventOne { x: 0.0, y: 0.0 };

  match x {
    ComplexEvent::A(ref a, _) => z = a,
    ComplexEvent::B(b, _) => trace!("{:?}", b)
  }

  trace!("Accessed values: {:?}", z);
}
