fn foo(_:isize) { trace!("func pointer"); }
type HasInt<'a> = &'a mut (FnMut(isize) + 'a);

#[test]
fn fp_test() {
   {
     let mut x:HasInt = &mut foo;
     x.call_mut((1is,));
   }
   {
     let mut y:HasInt = &mut |&mut:_:isize| { trace!("closure"); };
     y(2);
   }
}
