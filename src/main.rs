#![allow(unstable)]
#![feature(unboxed_closures)]

#[cfg(test)]
#[macro_use]
extern crate debugging;

#[cfg(test)]
pub use self::debugging::debug;

mod borrow;
mod cast_to_immutable;
mod cpus;
mod func_ptr;
mod generics;
mod imported;
mod imports;
mod indirect;
mod intrinsics;
mod iterators;
mod lifetime;
mod lifetime2;
mod lifetime3;
mod list;
mod map_test;
mod move_;
mod mutable_borrow;
mod nstrcmp;
mod output;
mod overload;
mod queue;
mod ressurect;
mod trait_ns;
mod traits;
mod traits_namespace;
mod tuples;
mod union;
mod vectors;
mod walk;
mod grapheme;
mod overload_function_with_generic;
mod vec2d;
mod search;
mod hash_test;
mod sized;
mod find_in_array;
mod function_pointers;

fn main() {
}
