//! Set and get program scheduling priority
use errno::{Errno, errno, set_errno};
use libc::{PRIO_PROCESS,PRIO_PGRP,PRIO_USER,setpriority,getpriority, id_t};

/// Which identifier type to use (`pid`, `gid`, or `uid`)
#[allow(missing_docs)]
pub enum Which {
    Process,
    Group,
    User,
}

/// Set the scheduling priority for the `Which` of the calling process
///
/// Priorities are usually in the range of -20..19, dependent on your system.
pub fn set_self_priority(which: Which, priority: i32) -> Result<(), ()> {
    set_priority(which, 0, priority)
}

/// Set the scheduling priority for the selected identifier (`pid`, `gid`, or `uid`)
///
/// Priorities are usually in the range of -20..19, dependent on your system.
pub fn set_priority(which: Which, who: i32, priority: i32) -> Result<(), ()> {
    let c_which = match which {
        Which::Process => PRIO_PROCESS,
        Which::Group => PRIO_PGRP,
        Which::User => PRIO_USER,
    };

    match unsafe { setpriority(c_which as u32, who as id_t, priority) } {
        0 => Ok(()),
        _ => Err(()),
    }
}

/// Get the scheduling priority for the `Which` of the calling process
pub fn get_self_priority(which: Which) -> Result<i32, ()> {
    get_priority(which, 0)
}

/// Get the scheduling priority for the selected identifier (`pid`, `gid`, or `uid`)
pub fn get_priority(which: Which, who: i32) -> Result<i32, ()> {
    let c_which = match which {
        Which::Process => PRIO_PROCESS,
        Which::Group => PRIO_PGRP,
        Which::User => PRIO_USER,
    };

    set_errno(Errno(0));
    let priority = unsafe { getpriority(c_which as u32, who as id_t) };
    match errno().0 {
        0 => Ok(priority),
        _ => Err(()),
    }
}
