// The first part is a doc comment. Normally they start with `///` on each
// line and then apply to the item below them, but `//!` means "document
// the outer thing instead". This is the normal way to document a module.
//  We don't have much to say here yet, but we might have stuff to say
// about other modules later on.
//
// Next we have a few attributes, which are normally like `#[thing]` and
// then document something below them, but again with the `!` in there they
// instead apply an attribute to the outer thing, in this case the module
// they're in. Since this module is also the root of the entire program,
// they apply to out program as a whole.
//
// - The first is a window specific attribute so that when we se release
//   mode it'll compile our program as a windows mode program (as opposed
//   to a console mode program). On non-windows this will just quitely 
//   have no effect.
// - The rest are bunch of directives to shut up compiler warnings. 
//   They're not all good in "production" code, but for getting something
//   on screen they're fine.

//! The main program!
//! 

///  These lines say that we'll use the `dwarf_term` crate, and then pull
/// in everything from the `dwarf_term` module which is the root of that
/// crate. The `std` crate doesn't need to be pulled in with "extern crate"
/// in a normal program, but I put a little comment line just so that when
/// we have several crates and each of their use statements, it all matches
/// up. From the standard library we'll want to use the `HashMap` and
/// `HashSet` types. Actually we'll only use `HashSet` right away, but
/// we'll end up with a `HashMap` soon enough I'm sure.

extern crate dwarf_term;
pub use drawf_term::*;

// std
use std::collections::{HashMap,HashSet};

///  The tiles for dwarf_term are each 12x12 pixels, and so with this many
/// grid cells we get a window that's around 800x600, which is a 
/// comfortable size on a large screen and still doesn't go off screen 
/// even on an older monitor. Like I said earlier, `dwarf_term` was put out
/// at the last minute to be available for this tutorial series, so the 0.1
/// version doesn't have configurable tilesets or anything like that. 
/// Someday I'll get around to that.

const TILE_GRID_WIDTH: usize = 66;
const TILE_GRID_HEIGHT: usize = 50;

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

}
