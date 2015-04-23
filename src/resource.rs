//! Set and get program scheduling priority
use ffi::resource::*;

/// Which identifier type to use (`pid`, `gid`, or `uid`)
#[allow(missing_docs)]
pub enum Which {
    Process,
    Group,
    User
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
        Which::Group => PRIO_GROUP,
        Which::User => PRIO_USER,
    };

    match unsafe { setpriority(c_which, who, priority) } {
        0 => Ok(()),
        _ => Err(())
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
        Which::Group => PRIO_GROUP,
        Which::User => PRIO_USER,
    };

    let priority = unsafe { getpriority(c_which, who) };
    Ok(priority)
}
