use _macros;

#[deriving(Show)]
enum List {
  Node(Box<List>), 
  Nil
}

fn count(list: &List) -> uint { 
  let mut y = box &list;
  let mut z = y;
  trace!("?? {}", z);
  match *list { 
    Node(box ref next) => 1+count(next), 
    Nil=> 0 
  } 
}

#[test]
fn test_list_things() {
  let x = Node(box Node(box Node(box Nil)));
  trace!("Count: {}", count(&x));
}
