use std::io::fs;
use std::io::FileType;

pub fn walk(path:&'static str) {
  let path = Path::new(path);
  let walker = fs::walk_dir(&path);
  if walker.is_ok() {
    for value in walker.ok().unwrap() {
      let tmp = Path::new(value.clone());
      let stats = fs::lstat(&tmp);
      match stats {
        Ok(stats) => {
            if stats.kind == FileType::Directory {
              println!("- Directory!");
            }
            else if stats.kind == FileType::RegularFile {
              println!("- File!");
              println!("- Last mod: {:?}", stats.modified);
            }
        },
        _ => {
          println!("Stat failed");
        }
      }
      println!("walk: {:?}", value.display());
    }
  }
  else {
      println!("walk: Invalid target: {:?}", path.display());
  }
}

#[test]
fn test_how_path_works() {
  let path = Box::new(Path::new("xxx"));
  let p2 = path.join("things").join("blah").join("dsfdsf");
  println!("{:?}", p2.display());
}

#[test]
fn test_walk_fails() {
  walk("gafdadsf");
}

#[test]
fn test_walk_works() {
  walk(".");
}
