trait Foo { fn foo(&self); }
struct Bar<T: ?Sized> { t: T }
impl Foo for usize { fn foo(&self) { println!("Hello"); } }

#[test]
fn test_sized() {
  let f:&Bar<Foo> = &(|| -> Bar<usize> { Bar { t: 0us }})();
  f.t.foo();
}
