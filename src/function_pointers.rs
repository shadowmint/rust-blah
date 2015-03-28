fn foo(_:isize) { trace!("func pointer"); }
type HasInt<'a> = &'a mut (FnMut(isize) + 'a);

#[test]
fn fp_test() {
   {
     let mut x:HasInt = &mut foo;
     x(1isize);
   }
   {
     let mut y:HasInt = &mut |_:isize| { trace!("closure"); };
     y(2);
   }
}
