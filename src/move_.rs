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
    List::Node(box ref next) => 1+count(next), 
    List::Nil=> 0 
  } 
}

#[test]
fn test_list_things() {
  let x = List::Node(box List::Node(box List::Node(box List::Nil)));
  trace!("Count: {}", count(&x));
}
