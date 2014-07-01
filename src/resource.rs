use std::mem::transmute;

struct Handle {
  valid: bool,
  r: *Resource
}

impl<'a> Handle {
  fn borrow(&mut self) -> &'a mut Resource {
    unsafe {
      return transmute(self.r);
    }
  }
}

#[deriving(Show)]
struct Resource {
  x: int
}

#[deriving(Show)]
struct ResourceManager {
  items: Vec<Resource>
}

impl ResourceManager {
  fn new() -> ResourceManager {
    return ResourceManager {
      items: Vec::<Resource>::new()
    };
  }
  fn add(&mut self, r: Resource) {
    self.items.push(r);
  }
  fn handle(& mut self, id: uint) -> Handle {
    let item:*Resource = self.items.get(id); 
    return Handle { valid: true, r: item };
  }
}

#[test]
fn test_resource() {
  let mut r = ResourceManager::new();
  r.add(Resource { x: 10 });
  r.add(Resource { x: 15 });
  r.add(Resource { x: 20 });

  // Specific scope
  let mut handle = r.handle(0);
  {
    let value = handle.borrow();
    value.x = 50;

    // Don't do this: actually modifying this means
    // you have two mutable references to one memory
    // location; resulting in undefined behaviour.
    // let value2 = handle.borrow();
    // value2.x = 100; // ADAFAF Undefined behaviour!
  }

  println!("{}", r);
}
