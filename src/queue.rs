struct Node {
  value:int,
  next:Option<~Node>
}

struct QQ {
  head: Option<~Node>,
  tail: Option<* mut ~Node>
}

impl QQ {
  fn push(& mut self, x:int) {
    match self.tail {
      Some(tp) => {
        let mut tail:& mut ~Node; 
        unsafe { tail = & mut (*tp); }
        tail.next = Some(~Node{ value: x, next: None });
        self.tail = Some(tail.next.get_mut_ref() as * mut ~Node);
      },
      None => {
        self.head = Some(~Node{ value: x, next: None });
        self.tail = Some(self.head.get_mut_ref() as * mut ~Node);
      }
    }
  }

  // Should probably be Result<int, Err> but I'm being lazy.
  fn pop(& mut self) -> int {
    if (self.head.is_none()) {
      return -1;
    }
    else {
      let local = self.head.take().unwrap();
      let rtn = local.value;
      self.head = local.next;
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
}
