//! Bindings to `sched.h` and `sys/resource.h`
//!
//! Just enough to set the scheduler priority.
#![deny(missing_docs)]
extern crate errno;
extern crate libc;

mod ffi;
mod sched;
mod resource;
mod cpuset;

pub use sched::*;
pub use resource::*;
pub use cpuset::CpuSet;
