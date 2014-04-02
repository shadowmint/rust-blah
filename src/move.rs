use _macros;

#[deriving(Show)]
enum List {
  Node(~List), 
  Nil
}

fn count(list: &List) -> uint { 
  let mut y = ~&list;
  let mut z = y;
  trace!("?? {}", z);
  match *list { 
    Node(~ref next) => 1+count(next), 
    Nil=> 0 
  } 
}

#[test]
fn test_list_things() {
  let x = Node(~Node(~Node(~Nil)));
  trace!("Count: {}", count(&x));
}
