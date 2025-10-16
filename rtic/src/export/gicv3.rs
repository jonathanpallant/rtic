use arm_gic::gicv3::GicCpuInterface;

pub mod interrupt {
    #[inline]
    pub fn disable() {
        cortex_ar::interrupt::disable();
    }

    #[inline]
    pub unsafe fn enable() {
        unsafe { cortex_ar::interrupt::enable() };
    }
}

pub fn pend() {
    
}

pub fn run<F>(priority: u8, f: F)
where
    F: FnOnce(),
{
    if priority == 1 {
        // If the priority of this interrupt is `1` then PMR can only be `0`
        f();
        GicCpuInterface::set_priority_mask(0);
    } else {
        let initial = GicCpuInterface::get_priority_mask();
        f();
        GicCpuInterface::set_priority_mask(initial);
    }
}

/// Lock implementation using GICv3 Priority Mask Register (PMR)
///
/// # Safety
///
/// The system ceiling is raised from current to ceiling
/// by raising the PMR to the ceiling value, or
#[inline(always)]
pub unsafe fn lock<T, R>(
    ptr: *mut T,
    ceiling: u8,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    let current = GicCpuInterface::get_priority_mask();
    // Only interrupts with a higher priority (numerically lower) will be signalled.
    // Priorities in RTIC follow a higher value = more important scheme, so invert.
    GicCpuInterface::set_priority_mask(u8::MAX - ceiling);
    let r = unsafe {
        f(&mut *ptr)
    };
    GicCpuInterface::set_priority_mask(current);
    r
}
