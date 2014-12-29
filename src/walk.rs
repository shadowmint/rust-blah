extern crate time;

use std::io::fs;
use std::io::FileType;
use self::time::Timespec;
use self::time::{at, strftime};
use _macros;

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
              println!("- Last mod: {}", stats.modified);
              let spec = Timespec::new((stats.modified / 1000) as i64, 0);
              let now = time::at(spec);
              println!("-- As tm: {}", strftime("%F - %H:%M:%S", &now));
            }
        },
        _ => {
          println!("Stat failed");
        }
      }
      println!("walk: {}", value.display());
    }
  }
  else {
      println!("walk: Invalid target: {}", path.display());
  }
}

#[test]
fn test_how_path_works() {
  let path = box Path::new("xxx");
  let p2 = path.join("things").join("blah").join("dsfdsf");
  println!("{}", p2.display());
}

#[test]
fn test_walk_fails() {
  walk("gafdadsf");
}

#[test]
fn test_walk_works() {
  walk(".");
}
