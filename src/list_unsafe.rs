use _macros;
use std::ptr::mut_null;

struct Node<T> {
  next:Option<~Node<T>>,
  data:Option<T>
}

impl<T:Clone> Node<T> {

  /** Create a new full node */
  fn new(value:T) -> Node<T> {
    return Node {
      next: None,
      data: Some(value)
    };
  }

  /** Create a new blank node */
  fn blank() -> Node<T> {
    return Node {
      next: None,
      data: None::<T>
    };
  }

  /** Attach a node as the 'next' node in this chain */
  fn push<'a>(&'a mut self, value: T) -> &'a mut ~Node<T> {
    let mut current = &self.next;
    let mut cref = & mut self;
    let mut looping = true;
    while looping {
      match *current {
        Some(ref x) => {
          current = & x.next;
          cref = & mut x;
        },
        None => looping = false
      }
    }
    cref.next = Some(~Node::<T>::new(value));
    return & mut cref.next.unwrap();
  }

  /* Print all items in the chain */
  fn print(& mut self, printer:|x:Option<T>|) {
    let mut current = self.next;
    let mut looping = true;
    while looping {
      match current {
        Some(ref x) => {
          current = x.next;
          printer(x.data);
        },
        None => looping = false
      }
    }
  }
}

#[test]
fn test_create_blank_node() {
  let mut x = Node::<int>::blank();
  x.push(10).push(11);
  x.print(|x:Option<int>| {
    match x {
      Some(ref x) => { trace!("Value: {}", x)},
      None => { trace!("Empty node"); }
    }
  });
}
