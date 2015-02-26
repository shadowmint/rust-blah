#![feature(core)]

use std::any::TypeId;
use std::raw::TraitObject;
use std::mem::transmute;

trait Fooish {
  fn as_type(&self) -> TypeId;
}

struct Foo<T: Copy> {
  #[allow(dead_code)]
  value:T
}

impl<T: Copy + 'static> Fooish for Foo<T> {
  fn as_type(&self) -> TypeId {
    return TypeId::of::<T>();
  }
}

fn maybe<T: 'static + Copy>(f:&Fooish) -> Option<T> {
  if TypeId::of::<T>() == f.as_type() {
    unsafe {
      let inner:TraitObject = transmute(f);
      let downcast:&Foo<T> = transmute(inner.data as *mut Foo<T>);
      return Some(downcast.value);
    }
  }
  return None;
}

#[test]
fn test_array() {
    let f1 = Foo { value: 1u8 };
    let f2 = Foo { value: 2u16 };
    let f3 = Foo { value: 3u32 };
    let f4 = Foo { value: "Hello" };
    let foo:Vec<&Fooish> = vec!(&f1, &f2, &f3, &f4);
    for f in foo {
        let mut matched = false;
        match maybe::<u8>(f) { Some(v) => { matched = true; println!("u8: {}", v); }, None => {} }
        match maybe::<u16>(f) { Some(v) => { matched = true; println!("u16: {}", v); }, None => {} }
        match maybe::<u32>(f) { Some(v) => { matched = true; println!("u32: {}", v); }, None => {} }
        if !matched {
            println!("Record had an unknown type");
        }
    }
}
