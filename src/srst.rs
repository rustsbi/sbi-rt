//! Chapter 10. System Reset Extension (EID #0x53525354 "SRST")

use crate::binary::{sbi_call_2, SbiRet};

pub use sbi_spec::srst::*;

/// §10.1
#[inline]
pub fn system_reset(reset_type: u32, reset_reason: u32) -> SbiRet {
    sbi_call_2(EID_SRST, SYSTEM_RESET, reset_type as _, reset_reason as _)
}
