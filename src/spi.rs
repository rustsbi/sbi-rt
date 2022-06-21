//! Chapter 7. IPI Extension (EID #0x735049 "sPI: s-mode IPI")

use crate::binary::{sbi_call_2, SbiRet};

pub use sbi_spec::spi::*;

/// §7.1
#[inline]
pub fn send_ipi(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call_2(EID_SPI, SEND_IPI, hart_mask, hart_mask_base)
}
