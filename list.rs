use _macros;
use std::cast;
use std::ptr;

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
    trace!("Next node: {:?}", self._next);
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
    trace!("Entered APPLY on target");
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
  let mut x = ~Node::new(10);
  x.push(11).push(12).push(13);
  x.each(|value:& mut int| {
    trace!("Got value: {:?}", value);
  });
}

/*
#[deriving(Show)] 
struct List<T> {
  _first: * mut Node<T>,
  _last: * mut Node<T>
}

impl<T> List<T> {

  /** Create a new list; invoke as: List::<T>::new() */
  fn new() -> List<T> {
    return List {
      _first: ptr::mut_null(),
      _last: ptr::mut_null()
    }
  }

  /** Push a new value onto the end of the list */
  fn push(&mut self, t:T) {
    if ptr::mut_null() == self._last {
      let mut node = Node::new(t);
      self._last = node.unsafe_ref();
      self._first = node.unsafe_ref();
      trace!("Set first pointer to ---> {}", self._first);
      trace!("Set root pointer to ---> {}", self._last);
    }
    else {
      unsafe {
        let mut node = Node::<T>::new(t);
        let link1 = node.unsafe_ref();
        let link2 = (*self._last).unsafe_ref();
        self._last = link1;
        node.set_prev(link2);
        trace!("Last ---> {}", self._last);
      }
    }
  }

  /** Run function on each list item */
  fn each(&mut self, worker:|t:&T|) {
    let mut here:* mut Node<T> = self._first;
    trace!("Entering EACH: {}", here);
    unsafe {
      let next_node = (*here).next();
    }
    //worker((*here).data());
  }
}

#[test]
fn test_create_list() {
  let list = List::<int>::new();
  //trace!("{}", list);
}

#[test]
fn test_add_elements_to_list() {
  let mut list = List::<int>::new();
  list.push(1);
  list.push(2);
  list.push(3);
  list.push(4);
  list.push(5);
  list.push(6);
  /*list.each(|t:&int| {
    trace!("EACH: {}", t);
  });*/
}*/
