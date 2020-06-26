use crate::Logger;

#[cfg(feature="rtt")]
use rtt_target::{
    rtt_init_print,
    print_impl::write_fmt
};

#[cfg(feature="rtt")]
#[allow(dead_code)]
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