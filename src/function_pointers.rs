fn foo(_:isize) -> () { trace!("func pointer"); }
type HasInt<'a> = &'a (FnMut<(isize), ()> + 'a);

#[test]
fn fp_test() {
   let x:HasInt = &foo;
   let y:HasInt = &|&mut:_:isize| { trace!("closure"); };
   x.call_mut(1); y.call_mut(2);
}
