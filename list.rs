use _macros;
use std::cast;
use std::ptr;

#[deriving(Show)] 
struct Node<T> {
  _prev:*Node<T>,
  _next:*Node<T>,
  _data:T
}

impl<T> Node<T> {

  /** Create a new Node holding the value T */
  fn new(t:T) -> Node<T> {
    return Node {
      _prev: ptr::null(),
      _next: ptr::null(),
      _data: t
    };
  }

  /** Return a pointer reference to this node */
  fn unsafe_ref<'a>(&'a self) -> *Node<T> {
    return self as *Node<T>;
  }

  /** Attach a node as the 'next' node in this chain */
  fn set_next(&mut self, node: *Node<T>) {
    self._next = node;
  }

  /** Attach a node as the 'prev' node in this chain */
  fn set_prev(&mut self, node: *Node<T>) {
    self._prev = node;
  }

  /** Get next node */
  fn next(&mut self) -> *Node<T> {
    return self._next;
  }
}

#[test]
fn test_create_node() {
  let x = Node::new(10);
  trace!("{}", x);
}

#[test]
fn test_create_node_chain() {
  let mut x = Node::new(10);
  let mut y = Node::new(11);
  let mut z = Node::new(12);
  x.set_next(y.unsafe_ref());
  y.set_prev(x.unsafe_ref());
  y.set_next(z.unsafe_ref());
  z.set_prev(y.unsafe_ref());
  trace!("X -> Y -> Z");
  trace!("X: {}", x);
  trace!("Y: {}", y);
  trace!("Z: {}", z);
}

#[deriving(Show)] 
struct List<T> {
  _first: *Node<T>,
  _last: *Node<T>
}

impl<T> List<T> {

  /** Create a new list; invoke as: List::<T>::new() */
  fn new() -> List<T> {
    return List {
      _first: ptr::null(),
      _last: ptr::null()
    }
  }

  /** Push a new value onto the end of the list */
  fn push(&mut self, t:T) {
    if ptr::null() == self._last {
      let node = Node::new(t);
      self._last = node.unsafe_ref();
      self._first = node.unsafe_ref();
    }
    else {
      unsafe {
        let mut node = Node::new(t);
        let link1 = node.unsafe_ref();
        let link2 = (*self._last).unsafe_ref();
        self._last = link1;
        node.set_prev(link2);
      }
    }
  }

  /** Run function on each list item */
  fn each(&mut self, worker:|t:T|) {
    let mut here:*Node<T> = self._first;
    while (ptr::null() != here) {
      unsafe {
        here = (*here).next();
      }
    }
  }
}

#[test]
fn test_create_list() {
  let list = List::<int>::new();
  trace!("{}", list);
}

#[test]
fn test_add_elements_to_list() {
  let mut list = List::<int>::new();
  list.push(1);
  list.push(2);
  list.push(3);
  trace!("{}", list);
}



/*// A super simple list type~
trait Array<T> {
  /*fn length(&mut self) -> int;
  fn push(&mut self, t:T);
  fn pop(&mut self) -> Result<T, ListError>;*/
  /*fn shift(&mut self, t:T);
  fn unshift(&mut self) -> T;
  fn indexOf(&mut self, t:T) -> int;
  fn filter(&mut self, filter: |T| -> bool);
  fn each(&mut self, filter: |T|);
  fn map(&mut self, filter: |T| -> T);
  fn search(&mut self, filter: |T| -> bool) -> ~Array<T>;
  fn first(&mut self, filter: |T| -> bool) -> T;
  fn all(&mut self) -> ~[T];*/
}

// Linked list node
#[deriving(Eq, Show)]
struct ListNode<T> {
  _next: Option<~ListNode<T>>,
  _prev: Option<*ListNode<T>>,
  _data: Option<T>
}

// Linked list node implementation
impl<T> ListNode<T> {

  // Create a new generic list
  // eg. let x:ListNode<ibt> = ListNode::new();
  // or  let x = ListNode::<int>::new();
  fn new() -> ListNode<T> {
    return ListNode { _data: None::<T>, _next: None::<~ListNode<T>>, _prev: None::<*ListNode<T>> };
  }

  // Set the data value for this node
  fn set_data(&mut self, x:T) {
    self._data = Some(x);
  }

  // Get the data value for this node 
  fn get_data<'a> (&'a mut self) -> &'a Option<T> {
    return &self._data;
  }

  // Set the previous pointer
  fn set_prev(&mut self, mut prev: &ListNode<T>) {
    unsafe {
      self._prev = Some(prev as *ListNode<T>);
    }
  }

  // Reset the previous pointer
  fn reset_prev(&mut self) {
    self._prev = None::<*ListNode<T>>;
  }

  // Create a new next node and return it
  fn extend_next<'a>(&'a mut self, value:T) -> Result<&'a ~ListNode<T>, ListError> {
    if (!self._next.is_none()) {
      let mut next = ~ListNode::<T>::new();
      next.set_data(value);
      next.set_prev(self);
      self._next = Some(next);
      return Ok(&next);
    }
    return Err(NotEmpty);
  }

  // Discard next element and all additional elements on the chain
  fn discard_next(&mut self) {
    self._next = None::<~ListNode<T>>;
  }
}

#[test]
fn test_create_list_node_instance() {
  let mut x:ListNode<int> = ListNode::new();
  x.set_data(10);
  x.extend_next(11);
  x.extend_next(12);
  x.discard_next();
  trace!("{}", x);
}

// Linked List<T> implementation
struct List<T> {
  _length: int,
  _first: Option<ListNode<T>>,
  _last: Option<ListNode<T>>
}

#[deriving(Eq)]
enum ListError {
  NoElements,
  NotEmpty
}

impl<T> List<T> {
  /*fn new<T>() -> ~Array<T> {
    return ~List<T> { _length: 0, _first: None, _last: None } as ~Array<T>;
  }*/
}

impl<T> Array<T> for List<T> {
}
  /*fn length(&mut self) -> int {
    return self._length;
  }
  fn push(&mut self, t:T) {
    self._length += 1;
  }
  fn pop(&mut self) -> Result<T, ListError> {
    if (self._length > 0) {
      let rtn = (self._last as Node)[1];
      self._length -= 1;
      self._last = self._last[0];
      if (self._last != Nil) {
        self._last[2] = Nil;
      }
      return Ok(rtn);;
    }
    return Err(NoElements);
  }*/
  /*fn shift<T>(&mut self, t:T) {
  _}
  fn unshift<T>(&mut self) -> T {
    return T;
  }
  fn indexOf(&mut self, t:T) -> int {
    return 0;
  }
  fn filter(&mut self, filter: |T| -> bool) {
  }
  fn each(&mut self, filter: |T|) {
  }
  fn map(&mut self, filter: |T| -> T) {
      return T;
  }
  fn search(&mut self, filter: |T| -> bool) -> ~Array<T> {
      return ~List<T> { ListNode<T> { Nil }};
  }
  fn first(&mut self, filter: |T| -> bool) -> T {
      return T;
  }
  fn all(&mut self) -> ~[T] {
      return ~[];
  }*/
*/
