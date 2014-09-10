trait Foo { fn foo(&self); }
struct Bar<Sized? T> { t: T }
impl Foo for uint { fn foo(&self) { println!("Hello"); } }

#[test]
fn test_sized() {
  let f:&Bar<Foo> = &(|| -> Bar<uint> { Bar { t: 0u }})();
  f.t.foo();
}
