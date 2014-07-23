extern crate time;

use std::io::fs;
use std::io::{TypeDirectory, TypeFile};
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
            if stats.kind == TypeDirectory {
              trace!("- Directory!");
            }
            else if stats.kind == TypeFile {
              trace!("- File!");
              trace!("- Last mod: {}", stats.modified);
              let spec = Timespec::new((stats.modified / 1000) as i64, 0);
              let now = time::at(spec);
              trace!("-- As tm: {}", strftime("%F - %H:%M:%S", &now));
            }
        },
        _ => {
          trace!("Stat failed");
        }
      }
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

#[test]
fn test_walk_works() {
  walk(".");
}
