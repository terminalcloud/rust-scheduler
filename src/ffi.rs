//! See `man sched.h` and `man resource.h`
pub mod sched {
    //! Bindings to `sched.h`
    use libc::*;

    // http://lxr.free-electrons.com/source/include/uapi/linux/sched.h#L37
    pub const SCHED_NORMAL: c_int = 0;
    pub const SCHED_FIFO: c_int = 1;
    pub const SCHED_RR: c_int = 2;
    pub const SCHED_BATCH: c_int = 3;
     /* SCHED_ISO: reserved but not implemented yet */
    pub const SCHED_IDLE: c_int = 5;
    pub const SCHED_DEADLINE: c_int = 6;

    #[repr(C)]
    pub struct SchedParam {
        pub priority: c_int
        // TODO: _POSIX_(THREAD)_SPORADIC_SERVER
    }

    #[link(name="c")]
    extern {
        pub fn sched_setscheduler(pid: pid_t, policy: c_int, param: *const SchedParam) -> c_int;
        pub fn sched_getscheduler(pid: pid_t) -> c_int;
        // TODO: Other fns guaranteed by `man sched.h`
    }
}

pub mod resource {
    //! Bindings to `sys/resource.h`
    use libc::*;

    // http://unix.superglobalmegacorp.com/Net2/newsrc/sys/resource.h.html
    pub const PRIO_PROCESS: c_int = 0;
    pub const PRIO_GROUP: c_int = 1;
    pub const PRIO_USER: c_int = 2;

    #[link(name="c")]
    extern {
        pub fn setpriority(which: c_int, who: c_int, priority: c_int) -> c_int;
        pub fn getpriority(which: c_int, who: c_int) -> c_int;
        // TODO: Other fns guaranteed by `man resource.h`
    }
}
