#[derive(Show)]
enum List {
  Node(Box<List>),
  Nil
}

fn count(list: &List) -> usize {
  let mut y = Box::new(&list);
  let mut z = y;
  trace!("?? {}", z);
  match *list {
    List::Node(ref next) => 1+count(next),
    List::Nil=> 0
  }
}

#[test]
fn test_list_things() {
  let x = List::Node(Box::new(List::Node(Box::new(List::Node(Box::new(List::Nil))))));
  trace!("Count: {}", count(&x));
}
