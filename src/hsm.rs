//! Chapter 9. Hart State Management Extension (EID #0x48534D "HSM")

use crate::binary::{sbi_call_0, sbi_call_1, sbi_call_3, SbiRet};

pub use sbi_spec::hsm::*;

/// §9.1
#[inline]
pub fn hart_start(hartid: usize, start_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(EID_HSM, HART_START, hartid, start_addr, opaque)
}

/// §9.2
#[inline]
pub fn hart_stop() -> SbiRet {
    sbi_call_0(EID_HSM, HART_STOP)
}

/// §9.3
#[inline]
pub fn hart_get_status(hartid: usize) -> SbiRet {
    sbi_call_1(EID_HSM, HART_GET_STATUS, hartid)
}

/// §9.4
#[inline]
pub fn hart_suspend(suspend_type: u32, resume_addr: usize, opaque: usize) -> SbiRet {
    sbi_call_3(
        EID_HSM,
        HART_SUSPEND,
        suspend_type as _,
        resume_addr,
        opaque,
    )
}
