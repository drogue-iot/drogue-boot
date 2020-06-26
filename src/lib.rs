// Licensed under the Apache-License 2.0

#![no_std]
#![feature(asm)]

#[cfg(feature = "rtt")]
pub mod rtt_logger;

use core::fmt::Arguments;

/// General logging SPI.
pub trait Logger {
    /// Log a formatted [core::fmt::Arguments] object.
    fn log(&self, _message: Arguments) -> () {}

    /// Log a string message.
    fn log_message(&self, message: &str) -> () {
        self.log(format_args!("{}\n", message));
    }
}

/// A no-op silent logger, used by default.
pub struct NoOpLogger {}

#[allow(dead_code)]
impl NoOpLogger {
    fn new() -> NoOpLogger {
        NoOpLogger {}
    }
}

impl Logger for NoOpLogger {}

static NOOP_LOGGER: NoOpLogger = NoOpLogger {};

#[allow(dead_code)]
pub struct Boot<'a> {
    boot_partition: u32,
    logger: &'a dyn Logger,
}

#[allow(dead_code)]
impl<'a> Boot<'a> {
    /// Create a new bootloader controller.
    /// * `partition` - The start address of the primary non-bootloader partition.
    pub fn new(partition: u32) -> Boot<'a> {
        Boot {
            boot_partition: partition,
            logger: &NOOP_LOGGER,
        }
    }

    /// Configure a logger to use by the bootloader.
    /// * `logger` The logger implementation to use.
    pub fn with_logger(&mut self, logger: &'a dyn Logger) -> &mut Self {
        self.logger = logger;
        self
    }

    /// Perform the bootloader boot sequence.
    pub fn boot(self: &Self) -> ! {
        self.logger.log_message("Drogue-IoT Bootloader");

        // The layout of the first two words within a partition
        // is the initial stack-pointer followed by the address
        // of the reset handler.
        //
        // Treat the parition as an address and do some pointer
        // arithmetic upon it to get a pointer to each u32.

        let sp: *const u32 = self.boot_partition as *const _;
        let reset: *const u32 = (self.boot_partition + 4) as *const _;

        let sp_val: u32;
        let reset_val: u32;

        unsafe {
            // Dereference the pointers to obtain the
            // actual values underneath them.
            sp_val = *sp as u32;
            reset_val = *reset as u32;
        }
        self.logger
            .log(format_args!("sp={} reset={}", sp_val, reset_val));

        do_jump(sp_val, reset_val)
    }
}

#[allow(dead_code)]
/// Perform the jump to the primary application to be executed.
extern "C" fn do_jump(_sp: u32, _reset: u32) -> ! {
    // C calling conventions places the first two args
    // in the first two registers:
    //
    // r0 = sp
    // r1 = reset

    unsafe {
        asm! {
            // Set the stack-pointer
            "msr msp, r0",
            // Branch to the reset handler.
            "blx r1",
        };
    }
    // not actually reached, but to satisfy the ! return value.
    loop {}
}
