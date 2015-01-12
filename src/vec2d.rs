use std::mem::transmute;
use std::ops::Index;

struct Vec2d {
  width:usize,
  values:Vec<usize>
}

struct Vec2dView {
  value:usize
}

impl Index<usize> for Vec2d {
  type Output = Vec2dView;
  fn index(&self, index:&usize) -> &Vec2dView {
    unsafe {
      return transmute(&self.values[self.width * *index]);
    }
  }
}

impl Index<usize> for Vec2dView {
  type Output = usize;
  fn index(&self, index:&usize) -> &usize {
    unsafe {
      return &(*((&self.value as *const usize).offset(*index as isize)));
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
