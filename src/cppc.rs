//! Chapter 14. CPPC Extension (EID #0x43505043 "CPPC")

#[cfg(target_pointer_width = "64")]
use crate::binary::sbi_call_2;
#[cfg(target_pointer_width = "32")]
use crate::binary::sbi_call_3;
use crate::binary::{sbi_call_1, SbiRet};
use sbi_spec::cppc::{EID_CPPC, PROBE, READ, READ_HI, WRITE};

#[inline]
pub fn cppc_probe(cppc_reg_id: usize) -> SbiRet {
    sbi_call_1(EID_CPPC, PROBE, cppc_reg_id)
}

#[inline]
pub fn cppc_read(cppc_reg_id: usize) -> SbiRet {
    sbi_call_1(EID_CPPC, READ, cppc_reg_id)
}

#[inline]
pub fn cppc_read_hi(cppc_reg_id: usize) -> SbiRet {
    sbi_call_1(EID_CPPC, READ_HI, cppc_reg_id)
}

#[inline]
pub fn cppc_write(cppc_reg_id: usize, value: u64) -> SbiRet {
    match () {
        #[cfg(target_pointer_width = "32")]
        () => sbi_call_3(EID_CPPC, WRITE, cppc_reg_id, value as _, (value >> 32) as _),
        #[cfg(target_pointer_width = "64")]
        () => sbi_call_2(EID_CPPC, WRITE, cppc_reg_id, value as _),
    }
}
