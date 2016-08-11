//! Bindings to `sched.h` and `sys/resource.h`
//!
//! Just enough to set the scheduler priority.
#![deny(missing_docs)]
extern crate errno;
extern crate libc;

mod sched;
mod resource;
#[cfg(any(target_os = "linux", target_os = "emscripten"))]
mod cpuset;

pub use sched::*;
pub use resource::*;
#[cfg(any(target_os = "linux", target_os = "emscripten"))]
pub use cpuset::CpuSet;
