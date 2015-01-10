fn foo(_:isize) { trace!("func pointer"); }
type HasInt = FnMut<isize, ()> + 'static;

#[test]
fn fp_test() {
   let x:HasInt = foo;
   let y:HasInt = |_:isize| { trace!("closure"); };
   x(1); y(2);
}
