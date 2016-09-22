use libc::{PRIO_PROCESS,PRIO_PGRP,PRIO_USER};

///! Set and get program scheduling priority
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
    platform::set_priority(c_which, who, priority)
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
    platform::get_priority(c_which, who)
}

mod platform {
    use errno::{Errno, errno, set_errno};
    use libc::{setpriority,getpriority};

    // glibc
    #[cfg(target_env="gnu")]
    pub fn get_priority(which: i32, who: i32) -> Result<i32, ()> {
        set_errno(Errno(0));
        let priority = unsafe { getpriority(which as u32, who as u32) };
        match errno().0 {
            0 => Ok(priority),
            _ => Err(()),
        }
    }

    #[cfg(target_env="gnu")]
    pub fn set_priority(which: i32, who: i32, priority: i32) -> Result<(), ()> {
        match unsafe { setpriority(which as u32, who as u32, priority) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    #[cfg(target_env="musl")]
    pub fn get_priority(which: i32, who: i32) -> Result<i32, ()> {
        set_errno(Errno(0));
        let priority = unsafe { getpriority(which, who as u32) };
        match errno().0 {
            0 => Ok(priority),
            _ => Err(()),
        }
    }

    #[cfg(target_env="musl")]
    pub fn set_priority(which: i32, who: i32, priority: i32) -> Result<(), ()> {
        match unsafe { setpriority(which, who as u32, priority) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    // FreeBSD
    #[cfg(target_os="freebsd")]
    pub fn get_priority(which: i32, who: i32) -> Result<i32, ()> {
        set_errno(Errno(0));
        let priority = unsafe { getpriority(which, who) };
        match errno().0 {
            0 => Ok(priority),
            _ => Err(()),
        }
    }

    #[cfg(target_os="freebsd")]
    pub fn set_priority(which: i32, who: i32, priority: i32) -> Result<(), ()> {
        match unsafe { setpriority(which, who, priority) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    // OS X
    #[cfg(target_os="macos")]
    pub fn get_priority(which: i32, who: i32) -> Result<i32, ()> {
        set_errno(Errno(0));
        let priority = unsafe { getpriority(which, who as u32) };
        match errno().0 {
            0 => Ok(priority),
            _ => Err(()),
        }
    }

    #[cfg(target_os="macos")]
    pub fn set_priority(which: i32, who: i32, priority: i32) -> Result<(), ()> {
        match unsafe { setpriority(which, who as u32, priority) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }
}
