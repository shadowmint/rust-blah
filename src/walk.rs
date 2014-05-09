use std::io::fs;
use _macros;

pub fn walk(path:&'static str) {
  let path = box Path::new(path);
  let walker = fs::walk_dir(path);
  if walker.is_ok() {
    for value in walker.ok().unwrap() {
      trace!("walk: {}", value.display());
    }
  }
  else {
      trace!("walk: Invalid target: {}", path.display());
  }
}

#[test]
fn test_how_path_works() {
  let path = box Path::new("xxx");
  let p2 = path.join("things").join("blah").join("dsfdsf");
  trace!("{}", p2.display());
}

#[test]
fn test_walk_fails() {
  walk("gafdadsf");
}
