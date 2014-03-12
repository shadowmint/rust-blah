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
    if (self._next.is_none()) {
      self._push_node(~Node::new(value));
    }
    else {
      let tmp = self._next.take().unwrap();
      let mut next = ~Node::new(value);
      next._push_node(tmp);
      self._push_node(next);
    }
    return self._next.as_mut().unwrap();
  }

  /** Attach a raw node object as the 'next' node in this chain */
  fn _push_node(& mut self, value: ~Node<T>) {
    self._next = Some(value);
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
  fn each<U>(&mut self, context:& mut U, c:|context: & mut U, value: & mut T|) {
    Node::_applyEach(context, self.data(), |x, y| c(x, y));
    let mut busy = true;
    let mut here = self.next();
    while busy {
      match here {
        Ok(e) => {
          Node::_applyEach(context, e.data(), |x, y| c(x, y));
          here = e.next();
        },
        _ => busy = false
      }
    }
  }

  /** Actually apply an each function to a data point */
  fn _applyEach<U>(context:& mut U, r:Result<& mut T, NodeErr>, c:|context:& mut U, value: & mut T|) {
    match r {
      Ok(e) => { 
        trace!("Walking down node: {:?}", e);
        c(context, e) 
      },
      _ => {}
    }
  }

  /** Walk the entire chain and count values */
  fn count(&mut self) -> int {
    let mut count = 0;
    self.each(& mut count, |c:& mut int, _:& mut T| {
      *c += 1;
    });
    return count;
  }

  /* Return an immutable filtered set of values */
  fn count_filtered(&mut self, c:|value: & T| -> bool) -> int {
    let mut count = 0;
    self.each(& mut count, |c:& mut int, _:& mut T| {
      *c += 1;
    });
    return count;
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
  trace!("Found a total of {:?} values", x.count());
  assert!(x.count() == 4);
}

#[test]
fn test_count_filtered_node_chain() {
  trace!("test_count_filtered_node_chain -->");
  let mut x = ~Node::<int>::blank();
  for i in range(0, 10) {
    x.push(i);
  }
  trace!("test_count_filtered_node_chain: Found a total of {:?} values", x.count());
}
