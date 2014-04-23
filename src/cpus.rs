extern crate libc;

macro_rules! trace(
  ($($arg:tt)*) => (
    { let x = ::std::io::stdout().write_line(format_args!(::std::fmt::format, $($arg)*)); println!("{}", x); }
  );
)

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
