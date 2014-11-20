use _macros;
use std::mem::swap;

#[deriving(Show)]
struct Node<T> {
  next:Option<Box<Node<T>>>,
  data:Option<T>
}

enum NodeErr {
  Nope
}

impl<T:Clone> Node<T> {

  /** Create a new Node holding the value T */
  fn new(t:T) -> Node<T> {
    return Node {
      next: None,
      data: Some(t)
    };
  }

  /** Create a new blank node */
  fn blank() -> Node<T> {
    return Node {
      next: None,
      data: None::<T>
    };
  }

  /** Create a new chain from a vector */
  fn import(t:Vec<T>) -> Node<T> {
    let mut rtn = Node::<T>::blank();
    for value in t.iter() {
      rtn.push(value.clone());
    }
    return rtn;
  }

  /** Attach a node as the 'next' node in this chain */
  fn push<'a>(&'a mut self, value: T) -> &'a mut Box<Node<T>> {
    match self.next.take() {
      None => self._push_node(box Node::new(value)),
      Some(v) => {
        let mut next = box Node::new(value);
        next._push_node(v);
        self._push_node(next);
      }
    }
    match self.next {
      Some(ref mut t) => t,
      None => unreachable!(),
    }
  }

  /** Attach a node as the new first node in this chain */
  fn unshift<'a>(&'a mut self, value: T) -> &'a mut Node<T> {
    let mut first:Node<T> = Node::new(value);
    swap(self, & mut first);
    self.next = Some(box first);
    return self;
  }

  /** Attach a raw node object as the 'next' node in this chain */
  fn _push_node(& mut self, value: Box<Node<T>>) {
    self.next = Some(value);
  }

  /** Get the 'next' node of this chain */
  fn next<'a>(&'a mut self) -> Result<&'a mut Box<Node<T>>, NodeErr> {
    match self.next.as_mut() {
      Some(e) => return Ok(e),
      None => return Err(NodeErr::Nope)
    }
  }

  /** Return data instance */
  fn data<'a>(&'a mut self) -> Result<&'a mut T, NodeErr> {
    match self.data {
      Some(ref mut e) => return Ok(e),
      None => return Err(NodeErr::Nope)
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
    self.each(& mut count, |count:& mut int, value:& mut T| {
      if c(value) {
        *count += 1;
      }
    });
    return count;
  }

  /* Return a mutable filtered vector of values */
  fn filter<'a>(&mut self, c:|value: & T| -> bool) -> Vec<T> {
    let mut rtn:Vec<T> = Vec::new();
    self.each(& mut rtn, |v:& mut Vec<T>, value:& mut T| {
      if c(value) {
        v.push((*value).clone());
      }
    });
    return rtn;
  }

  /* Return the nth entry in the list */
  fn index<'a>(&'a mut self, index:int) -> Option<Box<T>> {
    let mut rtn:Option<Box<T>> = None;
    {
      let mut count = 0;
      self.each(& mut count, |count:& mut int, value:& mut T| {
        let mut found = false;
        {
          if *count == index {
            found = true;
          }
          *count += 1;
        }
        if found {
          rtn = Some(box value.clone());
        }
      });
    }
    return rtn;
  }
}

#[test]
fn test_create_node() {
  let mut x = Node::new(10i);
  assert!(x.count() == 1);
}

#[test]
fn test_create_blank_node() {
  let mut x = Node::<int>::blank();
  assert!(x.count() == 0);
}

#[test]
fn test_create_from_vector() {
  let mut value = Vec::<int>::new();
  for x in range(0, 10) { value.push(x); }
  let mut x = Node::import(value);
  assert!(x.count() == 10);
}

#[test]
fn test_create_node_chain() {
  let mut x = box Node::<int>::blank();
  x.push(10).push(11).push(12).push(13);
  assert!(x.count() == 4);
}

#[test]
fn test_count_filtered_node_chain() {
  let mut x = box Node::<int>::blank();
  for i in range(0, 20) { x.push(i); }
  assert!(x.count_filtered(|v:&int| -> bool { return *v < 5; }) == 5);
  assert!(x.count_filtered(|v:&int| -> bool { return *v == 10; }) == 1);
  assert!(x.count_filtered(|v:&int| -> bool { return *v >= 5 && *v <= 10; }) == 6);
}

#[test]
fn test_filter_node_chain() {
  let mut x = box Node::<int>::blank();
  for i in range(0, 20) { x.push(i); }
  let output = x.filter(|v:&int| -> bool { return *v >= 5 && *v <= 10; });
  for i in range(5, 10) {
    assert!(output.contains(&i));
  }
}

#[test]
fn test_unshift() {
  let mut x = box Node::<int>::blank();
  x.push(10).push(11).push(12).push(13);
  x.unshift(9).unshift(8).unshift(7);
  assert!(*x.index(0).unwrap() == 7);
  assert!(*x.index(1).unwrap() == 8);
  assert!(*x.index(5).unwrap() == 12);
  assert!(*x.index(6).unwrap() == 13);
}
