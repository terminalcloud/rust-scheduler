//! A CPU bitmask implementation to be used with the sched_[gs]etaffinity functions.

use libc::{c_void, cpu_set_t, sched_getaffinity, sched_setaffinity};
use std::mem;
use std::ptr;
use std::cmp;

type Mask = u64;
const MASK_BITS: usize = 64;

/// Struct representing a bitmask to be used with the affinity functions.
/// Meant to represent the `CPU_*` macros from `sched.h`
pub struct CpuSet {
    mask: Vec<Mask>,
}

impl CpuSet {
    /// Create a new `CpuSet` with room for `num_cpus` CPUs, no cpu will be active.
    /// Equivalent of `CPU_ALLOC`
    pub fn new(num_cpus: usize) -> CpuSet {
        let elements = (num_cpus + MASK_BITS - 1) / MASK_BITS;
        let mask = vec![0; cmp::max(elements, 1)];
        CpuSet { mask: mask }
    }

    /// Create a new `CpuSet` from a given mask. For example a u64 or a u8.
    pub fn from_mask<T>(mask: T) -> CpuSet {
        let mut cpuset = Self::new(8 * mem::size_of::<T>());
        unsafe {
            ptr::write(cpuset.mut_mask_ptr() as *mut T, mask);
        }
        cpuset
    }

    /// Create a new `CpuSet` that with one CPU set as active.
    /// Shorthand for using `new` and `set`
    pub fn single(cpu: usize) -> CpuSet {
        let mut cpuset = Self::new(cpu + 1);
        cpuset.set(cpu);
        cpuset
    }

    /// Activate a given `cpu` on this `CpuSet`.
    /// If the given `cpu` does not fit in the current `CpuSet`, it will be expanded to fit.
    /// Equivalent of `CPU_SET`
    pub fn set(&mut self, cpu: usize) {
        let elem = cpu / MASK_BITS;
        let bit = cpu % MASK_BITS;
        while elem > self.mask.len() {
            self.mask.push(0);
        }
        self.mask[elem] |= 1 << bit;
    }

    /// Clear a given `cpu` on this `CpuSet`.
    /// If the given `cpu` does not fit within the current `CpuSet` nothing will happen.
    /// Equivalent of `CPU_CLR`.
    pub fn clear(&mut self, cpu: usize) {
        let elem = cpu / MASK_BITS;
        let bit = cpu % MASK_BITS;
        if elem < self.mask.len() {
            self.mask[elem] ^= 1 << bit;
        }
    }

    /// Get if a given CPU is active in this `CpuSet`.
    /// If `cpu` does not fit in this `CpuSet` false will be returned.
    /// Equivalent of `CPU_ISSET`.
    pub fn is_set(&self, cpu: usize) -> bool {
        let elem = cpu / MASK_BITS;
        let bit = cpu % MASK_BITS;
        if elem > self.len() {
            false
        } else {
            self.mask[elem] & (1 << bit) != 0
        }
    }

    /// Get the number of bytes in the mask.
    /// Produces the same results as `CPU_ALLOC_SIZE`.
    pub fn len(&self) -> usize {
        (MASK_BITS / 8) * self.mask.len()
    }

    /// Get the raw pointer to the bitmask
    /// Any modification of the `CpuSet` after this call might invalidate the pointer.
    pub fn mask_ptr(&self) -> *const c_void {
        self.mask.as_ptr() as *const c_void
    }

    /// Get a mutable raw pointer to the bitmask.
    /// Any modification of the `CpuSet` after this call might invalidate the pointer.
    pub fn mut_mask_ptr(&mut self) -> *mut c_void {
        self.mask.as_mut_ptr() as *mut c_void
    }

    /// Represent this `CpuSet` as a `u64`.
    /// Will return an `Err` if the `CpuSet` is too large to be written to a `u64`
    pub fn as_u64(&self) -> Result<u64, ()> {
        let src_size = self.len();
        let out_size = mem::size_of::<u64>();
        if src_size > out_size {
            Err(())
        } else {
            let mut mask: u64 = 0;
            unsafe {
                ptr::copy(self.mask_ptr(),
                          (&mut mask) as *mut _ as *mut c_void,
                          src_size)
            }
            Ok(mask)
        }
    }

    /// Sets the affinity described by this `CpuSet` to a given `pid`.
    pub fn set_affinity(&self, pid: i32) -> Result<(), ()> {
        match unsafe { sched_setaffinity(pid, self.len(), self.mask_ptr() as *const cpu_set_t) } {
            0 => Ok(()),
            _ => Err(()),
        }
    }

    /// Fetch the affinity for a given `pid` as a `CpuSet`.
    pub fn get_affinity(pid: i32, num_cpus: usize) -> Result<CpuSet, ()> {
        let mut cpuset = CpuSet::new(num_cpus);
        match unsafe {
            sched_getaffinity(pid, cpuset.len(), cpuset.mut_mask_ptr() as *mut cpu_set_t)
        } {
            0 => Ok(cpuset),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::BitXor;
    use super::{CpuSet, MASK_BITS};

    #[test]
    fn test_new_one_byte() {
        let mut cpuset = CpuSet::new(7);
        assert_eq!(MASK_BITS / 8, cpuset.len());
        assert_eq!(0, cpuset.as_u64().unwrap());
        cpuset = CpuSet::new(1);
        assert_eq!(MASK_BITS / 8, cpuset.len());
        assert_eq!(0, cpuset.as_u64().unwrap());
        cpuset = CpuSet::new(0);
        assert_eq!(MASK_BITS / 8, cpuset.len());
        assert_eq!(0, cpuset.as_u64().unwrap());
    }

    #[test]
    fn test_new_many_bytes() {
        let cpuset = CpuSet::new(125);
        assert_eq!(16, cpuset.len());
    }

    #[test]
    fn test_from_mask_u8() {
        let cpuset = CpuSet::from_mask(0x3 as u8);
        assert_eq!(3, cpuset.as_u64().unwrap());
    }

    #[test]
    fn test_from_mask_u64() {
        let mask: u64 = 1 << 60;
        let cpuset = CpuSet::from_mask::<u64>(mask);
        assert_eq!(8, cpuset.len());
        assert_eq!(mask, cpuset.as_u64().unwrap());
    }

    #[test]
    fn test_single_low() {
        let mask: u64 = 1 << 3;
        let cpuset = CpuSet::single(3);
        assert_eq!(MASK_BITS / 8, cpuset.len());
        assert_eq!(mask, cpuset.as_u64().unwrap());
    }

    #[test]
    fn test_single_high() {
        let cpuset = CpuSet::single(29);
        println!("vafan: {}", cpuset.as_u64().unwrap());
        for i in 0..MASK_BITS {
            println!("i: {}", i);
            assert!(cpuset.is_set(i).bitxor(i != 29));
        }
    }

    #[test]
    fn test_set_and_is_set() {
        let max = 100;
        let mut cpuset = CpuSet::new(max);
        for i in 0..max {
            for j in 0..max {
                assert!(cpuset.is_set(j).bitxor(j >= i));
            }
            cpuset.set(i);
        }
        for i in 0..max {
            assert!(cpuset.is_set(i));
        }
    }

    #[test]
    fn test_clear_and_is_set() {
        let mut cpuset = CpuSet::from_mask(::std::u64::MAX);
        let max = 64;
        for i in 0..max {
            for j in 0..max {
                assert!(cpuset.is_set(j).bitxor(j < i));
            }
            cpuset.clear(i);
        }
        for i in 0..max {
            assert!(!cpuset.is_set(i));
        }
    }

    #[test]
    fn test_is_set_too_large() {
        let cpuset = CpuSet::from_mask(0b11111111);
        assert!(!cpuset.is_set(9));
        assert!(!cpuset.is_set(10000));
    }

    #[test]
    fn test_as_u64_too_large_set() {
        let cpuset = CpuSet::new(80);
        assert!(cpuset.as_u64().is_err());
    }

    #[test]
    fn test_as_u64() {
        let mask: u16 = 0xf0ac;
        let cpuset = CpuSet::from_mask::<u16>(mask);
        assert_eq!(mask as u64, cpuset.as_u64().unwrap());
    }
}
