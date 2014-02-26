use std::io::fs;

fn main() {
  walk(~".");
  walk(~"Sfdsdf");
}

fn walk(path:~str) {
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
