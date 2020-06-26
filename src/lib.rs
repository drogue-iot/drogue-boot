#![no_std]
#![feature(asm)]

#[cfg(feature="rtt")]
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
    pub fn with_logger(&mut self, logger: &'a dyn Logger) -> &mut Self {
        self.logger = logger;
        self
    }

    /// Perform the bootloader boot sequence.
    pub fn boot(self: &Self) -> ! {
        self.logger.log_message("Drogue-IoT Bootloader");

        let sp: *const u32 = self.boot_partition as *const _;
        let reset: *const u32 = (self.boot_partition + 4) as *const _;

        unsafe {
            let sp_val: u32 = *sp as u32;
            let reset_val: u32 = *reset as u32;
            self.logger
                .log(format_args!("sp={} reset={}", sp_val, reset_val));
            do_jump(sp_val, reset_val)
        }
    }
}

#[allow(dead_code)]
unsafe extern "C" fn do_jump(_sp: u32, _reset: u32) -> ! {
    asm! {
            "msr msp, r0",
    };
    asm! {
            "blx r1"
    };
    loop {}
}
