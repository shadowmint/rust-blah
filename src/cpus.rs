use std::os::num_cpus;

#[test]
fn test_this_thing() {
  trace!("Cpus: {:?}", num_cpus());
}
