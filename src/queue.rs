struct Node {
  value:isize,
  next:Option<Box<Node>>
}

struct QQ {
  head: Option<Box<Node>>,
  tail: Option<* mut Box<Node>>
}

impl QQ {
  fn push(& mut self, x:isize) {
    match self.tail {
      Some(tp) => {
        let mut tail:& mut Box<Node>;
        unsafe { tail = & mut (*tp); }
        tail.next = Some(Box::new(Node{ value: x, next: None }));
        self.tail = Some(tail.next.as_mut().unwrap() as * mut Box<Node>);
      },
      None => {
        self.head = Some(Box::new(Node{ value: x, next: None }));
        self.tail = Some(self.head.as_mut().unwrap() as * mut Box<Node>);
      }
    }
  }

  // Should probably be Result<isize, Err> but I'm being lazy.
  fn pop(& mut self) -> isize {
    if self.head.is_none() {
      return -1;
    }
    else {
      let local = self.head.take().unwrap();
      let rtn = local.value;
      self.head = local.next;
      if self.head.is_none() {
        self.tail = None;
      }
      return rtn;
    }
  }
}

#[test]
fn test_push() {
  let mut q = QQ { head: None, tail: None };
  q.push(1);
  q.push(2);
  q.push(3);
  q.push(4);
}

#[test]
fn test_pop() {
  let mut q = QQ { head: None, tail: None };
  q.push(1);
  q.push(2);
  q.push(3);
  q.push(4);
  assert!(q.pop() == 1);
  assert!(q.pop() == 2);
  assert!(q.pop() == 3);
  assert!(q.pop() == 4);
  assert!(q.pop()== -1);

  q.push(1);
  assert!(q.pop()== 1);
}
