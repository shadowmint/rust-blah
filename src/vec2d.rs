use std::mem::transmute;
use std::ops::Index;

struct Vec2d {
  width:uint,
  values:Vec<uint>
}

struct Vec2dView {
  value:uint
}

impl Index<uint, Vec2dView> for Vec2d {
  fn index(&self, index:&uint) -> &Vec2dView {
    unsafe {
      return transmute(&self.values[self.width * *index]);
    }
  }
}

impl Index<uint, uint> for Vec2dView {
  fn index(&self, index:&uint) -> &uint {
    unsafe {
      return &(*((&self.value as *const uint).offset(*index as int)));
    }
  }
}

#[test]
fn test_2d() {
  let mut vec = Vec2d { values: Vec::with_capacity(100), width: 10 };
  for i in range(0, 100) {
    vec.values.push(i);
  }
  for i in range(0, 10) {
    for j in range(0, 10) {
      assert!(vec[i][j] == i * 10 + j);
    }
  }
}
