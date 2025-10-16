#![no_std]
#![no_main]

use rtic::app;

// systick_monotonic!(Mono, 1000);

#[app(peripherals = false, device = qemu_mps3_an536)]
mod app {

    #[local]
    struct LocalResources {
        local_counter: u32,
    }

    #[shared]
    struct SharedResources {
        shared_counter: u32,
    }

    #[init]
    fn init(_cx: init::Context) -> (SharedResources, LocalResources) {
        semihosting::println!("Init...");

        let shared = SharedResources {
            shared_counter: 100,
        };
        let local = LocalResources {
            local_counter: 200,
        };
        (shared, local)
    }

    #[idle(local = [local_counter])]
    fn idle(cx: idle::Context) -> ! {
        semihosting::println!("Idle...");
        loop {
            semihosting::println!("local_counter = {}", *cx.local.local_counter);
            *cx.local.local_counter += 1;
        }
    }

    /// Get from the cortex-r-rt start-up code into the RTIC generated main
    /// function.
    ///
    /// This is required because cortex-r-rt assumes your start-up function is
    /// called `kmain` not `main` (which is what RTIC generates).
    #[cortex_r_rt::entry]
    fn rt_main() -> ! {
        unsafe { main() }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    semihosting::println!("PANIC: {:?}", info);
    semihosting::process::exit(1);
}
