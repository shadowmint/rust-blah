#[derive(Debug)]
struct Gen<T> {
  data: Option<T>
}

impl<T> Gen<T> {
  fn new() -> Gen<T> {
    return Gen { data: None::<T> };
  }
}

#[test]
fn test_create_generic_new() {
  let x:Gen<isize> = Gen::new();
}

#[test]
fn test_create_generic_instance() {
  let x = Gen { data: Some(0is) };
  if (!x.data.is_none()) {
    trace!("Hello With int value: {:?}", x);
  }
}

#[test]
fn test_create_generic_instance_with_none() {
  let x = Gen { data: None::<isize> };
  if (x.data.is_none()) {
    trace!("No data found: {:?}", x);
  }
}
