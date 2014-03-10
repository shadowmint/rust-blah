use _macros;

#[deriving(Show)] 
struct Node<T> {
  _next:Option<~Node<T>>,
  _data:Option<T>
}

enum NodeErr {
  Nope
}

impl<T> Node<T> {

  /** Create a new Node holding the value T */
  fn new(t:T) -> Node<T> {
    return Node {
      _next: None,
      _data: Some(t)
    };
  }

  /** Create a new blank node */
  fn blank() -> Node<T> {
    return Node {
      _next: None,
      _data: None::<T>
    };
  }

  /** Attach a node as the 'next' node in this chain */
  fn push<'a>(&'a mut self, value: T) -> &'a mut ~Node<T> {
    self._next = Some(~Node::new(value));
    return self._next.as_mut().unwrap();
  }

  /** Get the 'next' node of this chain */
  fn next<'a>(&'a mut self) -> Result<&'a mut ~Node<T>, NodeErr> {
    match self._next.as_mut() {
      Some(e) => return Ok(e),
      None => return Err(Nope)
    }
  }

  /** Return data instance */
  fn data<'a>(&'a mut self) -> Result<&'a mut T, NodeErr> {
    match self._data {
      Some(ref mut e) => return Ok(e),
      None => return Err(Nope)
    }
  }

  /** Apply some operator to each element in the list*/
  fn each(&mut self, c:|value: & mut T|) {
    Node::_applyEach(self.data(), |x| c(x));
    let mut busy = true;
    let mut here = self.next();
    while busy {
      match here {
        Ok(mut e) => {
          Node::_applyEach(e.data(), |x| c(x));
          here = e.next();
        },
        _ => busy = false
      }
    }
  }

  /** Actually apply an each function to a data point */
  fn _applyEach(r:Result<& mut T, NodeErr>, c:|value: & mut T|) {
    match r {
      Ok(e) => { c(e) },
      _ => {}
    }
  }
}

#[test]
fn test_create_node() {
  let x = Node::new(10);
  trace!("{}", x);
}

#[test]
fn test_create_node_chain() {
  let mut x = ~Node::<int>::blank();
  x.push(10).push(11).push(12).push(13);
  x.each(|value:& mut int| {
    trace!("Got value: {:?}", value);
  });
}
