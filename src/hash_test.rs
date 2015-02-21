use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

struct Tmp {
  v:String
}

#[derive(Eq)]
struct TmpKey {
  key:*const String
}

impl Tmp {
  fn new(value:&str) -> Tmp {
    return Tmp {
      v:String::from_str(value)
    };
  }
  fn borrow(&self) -> TmpKey {
    return TmpKey { key: &self.v as *const String };
  }
}

impl Hash for TmpKey {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let tmp1:&String;
    unsafe {
      tmp1 = &(*self.key);
    }
    tmp1.hash(state);
  }
}

impl PartialEq for TmpKey {
  fn eq(&self, other: &TmpKey) -> bool {
    let tmp1:&String;
    let tmp2:&String;
    unsafe {
      tmp1 = &(*self.key);
      tmp2 = &(*other.key);
    }
    return *tmp1 == *tmp2;
  }
}

#[test]
fn test_hash_with_keys() {
  let mut map = HashMap::<TmpKey, Tmp>::new();
  let value = Tmp::new("Hello");
  let value2 = Tmp::new("World");
  map.insert(value.borrow(), value);
  map.insert(value2.borrow(), value2);
}
