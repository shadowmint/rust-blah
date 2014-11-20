use _macros;

#[deriving(Show)] 
enum MyErr {
  None,
  Fail,
}

fn returns_tuple() -> (int, MyErr) {
  // return (1, None);
  return (0, MyErr::Fail);
}

fn returns_result() -> Result<int, MyErr> {
  // return Ok(1);
  return Err(MyErr::Fail);
}

#[test]
fn test_check_return_values() {
  let x = returns_result();
  if x.is_ok() {
    trace!("result: Is OK: {}", x.unwrap());
  }
  else {
    match x.err().unwrap() {
      MyErr::None => {} // Required for match
      MyErr::Fail => {
        trace!("result: Failed One");
      }
    }
  }
}

#[test]
fn test_check_return_values_2() {
  let (y, err) = returns_tuple();
  match err {
    MyErr::None => { trace!("tuple: Is OK: {}", y); },
    MyErr::Fail => { trace!("tuple: Failed one"); }
  }
}
