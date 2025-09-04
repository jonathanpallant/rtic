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