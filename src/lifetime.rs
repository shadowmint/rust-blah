use std::fmt;

struct StateLF;

struct BlahLF {
  id: isize,
  state: StateLF
}

static mut _id:isize = 0;

fn new_id() -> isize {
  unsafe {
    _id += 1;
    return _id;
  }
}


impl BlahLF {
  fn new() -> BlahLF {
      let id = new_id();
      trace!("Created: {:?}", id);
      return BlahLF {
        id: id,
        state: StateLF
      };
  }
  fn state<'a>(&'a mut self) -> &'a StateLF {
    return &self.state;
  }
}

impl fmt::Debug for BlahLF {
  fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
    return write!(f, "<BlahLF:: ");
  }
}

impl Drop for BlahLF {
  fn drop(&mut self) {
    trace!("Dropped element: {:?}", *self);
  }
}

#[test]
fn test_lifetime_scope() {
  let mut x:Box<BlahLF> = Box::new(BlahLF::new());
  let y = x.state();
  let z = Box::new(BlahLF::new());

  // NB. We can't do this because x has a borrowed state
  // x = ~BlahLF::new();
}

struct HasValues {
  values: Vec<isize>
}

fn borrow_values<'a>(x:&'a HasValues) -> Vec<&'a isize> {
  let mut rtn:Vec<&'a isize> = Vec::new();
  for i in 0..x.values.len() {
    rtn.push(x.values.get(i).unwrap());
  }
  return rtn;
}

#[test]
fn test_lifetime_scope_again() {
  let mut q = HasValues { values: Vec::<isize>::new() };
  q.values.push(1);
  q.values.push(2);
  q.values.push(3);
  q.values.push(4);
  q.values.push(5);
  let mut p = borrow_values(&q);
  trace!("Output of lifetime test: {:?}", p);
}

fn borrow_values_that_match<'a, 'b>(x:&'a HasValues, filter:&'b Fn(isize) -> bool) -> Vec<&'a isize> {
  let mut rtn:Vec<&isize> = Vec::new();
  for i in 0..x.values.len() {
    if filter(*x.values.get(i).unwrap()) {
      rtn.push(x.values.get(i).unwrap());
    }
  }
  return rtn;
}

#[test]
fn test_lifetime_scope_with_filter() {
  let mut q = HasValues { values: vec!(1, 2, 3, 4, 5) };
  let mut p = borrow_values_that_match(&q, &|f:isize| -> bool { return f > 2; });
  trace!("Output of lifetime test: {:?}", p);
}
