// Licensed under the Apache-License 2.0

use crate::Logger;

#[cfg(feature="rtt")]
use rtt_target::{
    rtt_init_print,
    print_impl::write_fmt
};

#[cfg(feature="rtt")]
#[allow(dead_code)]
/// Logger which uses SEGGER RTT protocol to communicate
/// progress back to an attached host system.
/// 
/// This logger may be useful when using a crate
/// such as `cargo-embed`.
pub struct RTTLogger {

}

#[cfg(feature="rtt")]
#[allow(dead_code)]
impl RTTLogger {
    pub fn new() -> RTTLogger {
        rtt_init_print!();
        RTTLogger{}
    }

}

#[cfg(feature="rtt")]
impl Logger for RTTLogger {
    fn log(&self, message: core::fmt::Arguments) -> () {
        write_fmt(0, message);
    }
}