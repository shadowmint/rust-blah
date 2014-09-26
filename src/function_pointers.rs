fn foo(_:int) { trace!("func pointer"); }
type HasInt = |int|: 'static;

#[test]
fn fp_test() {
   let x:HasInt = foo;
   let y:HasInt = |_:int| { trace!("closure"); };
   x(1); y(2);
}
