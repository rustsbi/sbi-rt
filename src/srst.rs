//! Chapter 10. System Reset Extension (EID #0x53525354 "SRST")

use crate::binary::{sbi_call_2, SbiRet};

pub use sbi_spec::srst::*;

/// Reset the system based on provided `reset_type` and `reset_reason`.
/// 
/// This is a synchronous call and does not return if it succeeds.
/// 
/// # Warm reboot and cold reboot
///
/// When supervisor software is running natively, the SBI implementation is machine mode firmware.
/// In this case, shutdown is equivalent to physical power down of the entire system and
/// cold reboot is equivalent to physical power cycle of the entire system. Further, warm reboot
/// is equivalent to a power cycle of main processor and parts of the system but not the entire system.
///
/// For example, on a server class system with a BMC (board management controller),
/// a warm reboot will not power cycle the BMC whereas a cold reboot will definitely power cycle the BMC.
///
/// When supervisor software is running inside a virtual machine, the SBI implementation is a hypervisor.
/// The shutdown, cold reboot and warm reboot will behave functionally the same as the native case but might
/// not result in any physical power changes.
/// 
/// This function is defined in RISC-V SBI Specification chapter 10.1.
#[inline]
pub fn system_reset(reset_type: u32, reset_reason: u32) -> SbiRet {
    sbi_call_2(EID_SRST, SYSTEM_RESET, reset_type as _, reset_reason as _)
}
