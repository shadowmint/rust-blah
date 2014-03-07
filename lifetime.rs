use uuid::Uuid;
use std::fmt;

struct BlahLF {
  id: Uuid
}

struct StateLF;

impl BlahLF {
  fn new() -> BlahLF {
      let id = Uuid::new_v4();
      trace!("Created: {}", id);
      return BlahLF { 
        id: id,
        state: State
      };
  }
  fn state(&mut self) -> &State {
    return self.state;
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
  let x = BlahLF::new();
  let y = x.state();
  x = BlahLF::new();
}
