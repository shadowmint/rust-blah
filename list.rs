use _macros;

// A super simple list type~
trait List<T> {
  fn length(&mut self);
  fn push(&mut self, t:T);
  fn pop(&mut self);
  fn shift(&mut self);
  fn unshift(&mut self);
  fn indexOf(&mut self);
  fn filter(&mut self);
  fn each(&mut self);
  fn map(&mut self);
  fn search(&mut self);
  fn first(&mut self);
}

#[test]
fn test_create_instance() {
}
