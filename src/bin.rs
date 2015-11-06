/**
 * Debug print immediately to stdout
 * usage:
 *
 *    debug!("Hello: {:?}", 1);
 */
#[macro_export]
macro_rules! trace(
  {$($arg:tt)*} => {
    {
      use debug;
      use std::io::Write;
      let _ = debug::stdout().write_fmt(format_args!($($arg)*));
      let _ = debug::stdout().write(&['\n' as u8]);
    }
  };
);

pub mod debug;
pub mod borrow;
pub mod cast_to_immutable;
pub mod func_ptr;
pub mod generics;
pub mod imported;
pub mod imports;
pub mod indirect;
pub mod iterators;
pub mod lifetime;
pub mod lifetime2;
pub mod lifetime3;
pub mod map_test;
pub mod mutable_borrow;
pub mod overload;
pub mod queue;
pub mod trait_ns;
pub mod traits;
pub mod traits_namespace;
pub mod tuples;
pub mod union;
pub mod vectors;
pub mod overload_function_with_generic;
pub mod sized;
pub mod find_in_array;
pub mod function_pointers;
pub mod show_debug;
pub mod fixed_sized_arrays;

fn main() {
}
