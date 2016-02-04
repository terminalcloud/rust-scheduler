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
