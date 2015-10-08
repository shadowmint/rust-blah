use std::sync::{Once, ONCE_INIT};
use std::io::stdout as factory;
use std::io::Stdout;
use std::io::StdoutLock;
use std::mem::transmute;

static START:Once = ONCE_INIT;
static mut stdout_:*mut Stdout = 0 as *mut Stdout;

#[allow(dead_code)]
pub fn stdout<'a>() -> StdoutLock<'a> {
  START.call_once(|| {
    unsafe {
      stdout_ = transmute(Box::new(factory()));
    }
  });
  unsafe {
    return (*stdout_).lock();
  }
}

/// Debug print immediately to stdout
#[macro_export]
macro_rules! debug(
  {$($arg:tt)*} => {
    {
      use $crate::debug;
      use std::io::Write;
      let _ = debug::stdout().write_fmt(format_args!($($arg)*));
      let _ = debug::stdout().write(&['\n' as u8]);
    }
  };
);

#[cfg(test)]
mod tests {

  #[test]
  fn test_trace_macro() {
    debug!("----------:");
    debug!("Trace test: {} {}", 0u32, 2i32);
  }
}
