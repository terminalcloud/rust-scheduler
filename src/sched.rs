//! Set and get scheduling policies
use ffi::sched::*;
use cpuset::CpuSet;

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

/// Set the cpu affinity for the current thread See `set_affinity`.
pub fn set_self_affinity(cpuset: CpuSet) -> Result<(), ()> {
    set_affinity(0, cpuset)
}

/// Set the cpu affinity for a thread.
pub fn set_affinity(pid: i32, cpuset: CpuSet) -> Result<(), ()> {
    cpuset.set_affinity(pid)
}

/// Get the cpu affinity for the current thread. See `get_affinity`.
pub fn get_self_affinity(num_cpus: usize) -> Result<CpuSet, ()> {
    get_affinity(0, num_cpus)
}

/// Get the cpu affinity for a thread.
///
/// Create and return a `CpuSet` that has room for at least `num_cpus` and with those set
/// according to the current affinity.
pub fn get_affinity(pid: i32, num_cpus: usize) -> Result<CpuSet, ()> {
    CpuSet::get_affinity(pid, num_cpus)
}

#[cfg(test)]
mod tests {
    use super::{get_self_affinity, set_self_affinity};
    use cpuset::CpuSet;

    #[test]
    fn test_set_get_self_affinity() {
        let mask: u64 = 1; // CPU 0 only
        set_self_affinity(CpuSet::from_mask(mask)).unwrap();
        let read_mask = get_self_affinity(1).unwrap().as_u64().unwrap();
        assert_eq!(mask, read_mask);
    }


    #[test]
    fn test_set_get_self_affinity_2() {
        let mask: u64 = 1 << 0 | 1 << 1; // CPU 0 & 1
        set_self_affinity(CpuSet::from_mask(mask)).unwrap();
        let read_mask = get_self_affinity(2).unwrap().as_u64().unwrap();
        assert_eq!(mask, read_mask);
    }

    #[test]
    fn test_set_affinity_no_cpu() {
        assert!(set_self_affinity(CpuSet::new(0)).is_err());
    }
}
