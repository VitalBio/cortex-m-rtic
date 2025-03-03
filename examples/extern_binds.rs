//! examples/extern_binds.rs

#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(missing_docs)]
#![no_main]
#![no_std]

use cortex_m_semihosting::hprintln;
use panic_semihosting as _;

// Free function implementing the interrupt bound task `foo`.
fn foo(_: app::foo::Context) {
    hprintln!("foo called");
}

#[rtic::app(device = lm3s6965)]
mod app {
    use crate::foo;
    use cortex_m_semihosting::{debug, hprintln};
    use lm3s6965::Interrupt;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtic::pend(Interrupt::UART0);

        hprintln!("init");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        hprintln!("idle");

        rtic::pend(Interrupt::UART0);

        loop {
            cortex_m::asm::nop();
            // Exit moved after nop to ensure that rtic::pend gets
            // to run before exiting
            debug::exit(debug::EXIT_SUCCESS); // Exit QEMU simulator
        }
    }

    extern "Rust" {
        #[task(binds = UART0)]
        fn foo(_: foo::Context);
    }
}
