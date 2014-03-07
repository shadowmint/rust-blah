use uuid::Uuid;
use std::fmt;

struct StateLF;

struct BlahLF {
  id: Uuid,
  state: StateLF
}

impl BlahLF {
  fn new() -> BlahLF {
      let id = Uuid::new_v4();
      trace!("Created: {}", id);
      return BlahLF { 
        id: id,
        state: StateLF
      };
  }
  fn state<'a>(&'a mut self) -> &'a StateLF {
    return &self.state;
  }
}

impl fmt::Show for BlahLF {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f.buf, "<BlahLF:: {}>", self.id);
  }
}

impl Drop for BlahLF {
  fn drop(&mut self) {
    trace!("Dropped element: {}", *self);
  }
}

#[test]
fn test_lifetime_scope() {
  let mut x:~BlahLF = ~BlahLF::new();
  let y = x.state();
  let z = ~BlahLF::new();
  
  // NB. We can't do this because x has a borrowed state
  // x = ~BlahLF::new();
}
