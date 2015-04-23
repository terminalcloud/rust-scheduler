//! Bindings to `sched.h` and `sys/resource.h`
//!
//! Just enough to set the scheduler priority.
#![deny(missing_docs)]
extern crate libc;

mod ffi;
pub mod sched;
pub mod resource;

#[test]
fn it_works() {
}
