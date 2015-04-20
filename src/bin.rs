#[macro_use]
extern crate debugging;
pub use self::debugging::debug;

mod borrow;
mod cast_to_immutable;
mod func_ptr;
mod generics;
mod imported;
mod imports;
mod indirect;
mod iterators;
mod lifetime;
mod lifetime2;
mod lifetime3;
mod map_test;
mod mutable_borrow;
mod overload;
mod queue;
mod trait_ns;
mod traits;
mod traits_namespace;
mod tuples;
mod union;
mod vectors;
mod overload_function_with_generic;
mod sized;
mod find_in_array;
mod function_pointers;
mod show_debug;

fn main() {
}
