use _macros;
use std::cast;
use std::ptr;

#[deriving(Show)] 
struct Node<T> {
  _next:Option<~Node<T>>,
  _data:T
}

impl<T> Node<T> {

  /** Create a new Node holding the value T */
  fn new(t:T) -> Node<T> {
    return Node {
      _next: None,
      _data: t
    };
  }

  /** Attach a node as the 'next' node in this chain */
  fn set_next<'a>(&mut self, mut node: ~Node<T>) -> &'a mut ~Node<T> {
    //let x = &'a mut node;
    //self._next = Some(node);
    return & mut node;
  }

  /** Get next node */
  fn next<'a> (&'a mut self) -> Result<&'a mut ~Node<T>, int> {
    match self._next {
      Some(ref mut x) => return Ok(x),
      None => return Err(0)
    }
  }

  /** Return data instance */
  fn data<'a>(&'a mut self) -> &'a T {
    return &self._data;
  }
}

//#[test]
fn test_create_node() {
  let x = Node::new(10);
  trace!("{}", x);
}

//#[test]
fn test_create_node_chain() {
  let mut x = Node::new(10);
  let mut y = ~Node::new(11);
  let mut z = ~Node::new(12);
  x.set_next(y).set_next(z);
  /*match x.next() {
  match x.next() {
    Ok(mut e) => e.set_next(z),
    Err(_) => trace!("None")
  }
    Ok(mut i) => i.set_next(z),
    Err(_) => {}
  }*/
  /*z.set_prev(y.unsafe_ref());
  trace!("X -> Y -> Z");
  trace!("X: {}", x);
  trace!("Y: {}", y);
  trace!("Z: {}", z);*/
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
