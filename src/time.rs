//! Chapter 6. Timer Extension (EID #0x54494D45 "TIME")

use crate::SbiRet;

pub use sbi_spec::time::*;

/// §6.1
#[inline]
pub fn set_timer(stime_value: u64) -> SbiRet {
    match () {
        #[cfg(target_pointer_width = "32")]
        () => crate::binary::sbi_call_2(
            EID_TIME,
            SET_TIMER,
            stime_value as _,
            (stime_value >> 32) as _,
        ),
        #[cfg(target_pointer_width = "64")]
        () => crate::binary::sbi_call_1(EID_TIME, SET_TIMER, stime_value as _),
    }
}
