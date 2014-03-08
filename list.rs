use _macros;
use std::cast;
use std::ptr;

// A super simple list type~
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


// An unsafe pointer type
struct RawNode<T> {
  p: *mut T  
}

impl<T> RawNode<T> {
  fn None() -> RawNode<T> {
    return RawNode { p: ptr::mut_null() };
  }
  fn Some(t:T) -> RawNode<T> {
    let mut rtn = RawNode::<T>::None();;
    unsafe {
      let rtn = RawNode { p: cast::transmute_mut_unsafe::<T>(&t) };
    }
    return rtn;
  }
}


// Linked list node
#[deriving(Eq, Show)]
struct ListNode<T> {
  _next: Option<~ListNode<T>>,
  _data: Option<T>
}

// Linked list node implementation
impl<T> ListNode<T> {

  // Create a new generic list
  // eg. let x:ListNode<ibt> = ListNode::new();
  // or  let x = ListNode::<int>::new();
  fn new() -> ListNode<T> {
    return ListNode { _data: None::<T>, _next: None::<~ListNode<T>> };
  }

  // Set the data value for this node
  fn set(&mut self, x:T) {
    self._data = Some(x);
  }

  // Create a new next node and return it
  fn add(&mut self, value:T) {
    let mut next = ~ListNode::<T>::new();
    next.set(value);
    self._next = Some(next);
  }
}

#[test]
fn test_create_list_node_instance() {
  let mut x:ListNode<int> = ListNode::new();
  x.set(10);
  x.add(11);
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
  NoElements
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

