use _macros;

#[deriving(Eq, Show)] 
enum MyErr {
  None,
  FailOne,
}

fn returns_tuple() -> (int, MyErr) {
  // return (1, None);
  return (0, FailOne);
}

fn returns_result() -> Result<int, MyErr> {
  // return Ok(1);
  return Err(FailOne);
}

#[test]
fn test_check_return_values() {
  let x = returns_result();
  if x.is_ok() {
    trace!("result: Is OK: {}", x.unwrap());
  }
  else {
    match x.err().unwrap() {
      None => {} // Required for match
      FailOne => {
        trace!("result: Failed One");
      }
    }
  }
}

#[test]
fn test_check_return_values_2() {
  let (y, err) = returns_tuple();
  match err {
    None => { trace!("tuple: Is OK: {}", y); },
    FailOne => { trace!("tuple: Failed one"); }
  }
}
