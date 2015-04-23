//! Set and get scheduling policies
use ffi::sched::*;

/// Policies that may be set
///
/// Not all of these are supported by this binding.
/// Specifically, this binding was made to support a simple use of `RoundRobin`.
///
/// If you are considering another policy, consider updating this source as well.
#[allow(missing_docs)]
pub enum Policy {
    Normal,
    Fifo,
    RoundRobin,
    Batch,
    Idle,
    Deadline,
}

/// Set the scheduling policy for this process
pub fn set_self_policy(policy: Policy, priority: i32) -> Result<(), ()> {
    set_policy(0, policy, priority)
}

/// Set the scheduling policy for a process
pub fn set_policy(pid: i32, policy: Policy, priority: i32) -> Result<(), ()> {
    let c_policy = match policy {
        Policy::Normal => SCHED_NORMAL,
        Policy::Fifo => SCHED_FIFO,
        Policy::RoundRobin => SCHED_RR,
        Policy::Batch => SCHED_BATCH,
        Policy::Idle => SCHED_IDLE,
        Policy::Deadline => SCHED_DEADLINE
    };
    let params = SchedParam { priority: priority };
    let params_ptr: *const SchedParam = &params;

    match unsafe { sched_setscheduler(pid, c_policy, params_ptr) } {
        0 => Ok(()),
        _ => Err(())
    }
}

/// Get the scheduling policy for this process
pub fn get_self_policy() -> Result<Policy, ()> {
    get_policy(0)
}

/// Get the scheduling policy for a process
pub fn get_policy(pid: i32) -> Result<Policy, ()> {
    match unsafe { sched_getscheduler(pid) } {
        SCHED_NORMAL => Ok(Policy::Normal),
        SCHED_FIFO => Ok(Policy::Fifo),
        SCHED_RR => Ok(Policy::RoundRobin),
        SCHED_BATCH => Ok(Policy::Batch),
        SCHED_IDLE => Ok(Policy::Idle),
        SCHED_DEADLINE => Ok(Policy::Deadline),
        -1 => Err(()),
        policy @ _ => panic!("Policy {} does not exist", policy)
    }
}
