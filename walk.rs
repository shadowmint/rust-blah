use std::io::fs;

pub fn walk(path:~str) {
  let path = ~Path::new(path);
  let walker = fs::walk_dir(path);
  if walker.is_ok() {
    for value in walker.ok().unwrap() {
      println!("{}", value.display());
    }
  }
  else {
    println!("Invalid target");
  }
}

#[test]
fn test_walk_works() {
  walk(~".");
}

#[test]
fn test_walk_fails() {
  walk(~"gafdadsf");
}
