extern crate libc;

use _macros;

pub fn num_cpus() -> uint {
    unsafe {
        return rust_get_num_cpus();
    }
    extern {
        fn rust_get_num_cpus() -> libc::uintptr_t;
    }
}

#[test]
fn test_this_thing() {
  trace!("Cpus: {}", num_cpus());
}
