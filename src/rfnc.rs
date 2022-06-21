//! Chapter 8. RFENCE Extension (EID #0x52464E43 "RFNC")

use crate::binary::{sbi_call_2, sbi_call_4, sbi_call_5, SbiRet};

pub use sbi_spec::rfnc::*;

/// §8.1
#[inline]
pub fn remote_fence_i(hart_mask: usize, hart_mask_base: usize) -> SbiRet {
    sbi_call_2(EID_RFNC, REMOTE_FENCE_I, hart_mask, hart_mask_base)
}

/// §8.2
#[inline]
pub fn remote_sfence_vma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    sbi_call_4(
        EID_RFNC,
        REMOTE_SFENCE_VMA,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
    )
}

/// §8.3
#[inline]
pub fn remote_sfence_vma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> SbiRet {
    sbi_call_5(
        EID_RFNC,
        REMOTE_SFENCE_VMA_ASID,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
        asid,
    )
}

/// §8.4
#[inline]
pub fn remote_hfence_gvma_vmid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    vmid: usize,
) -> SbiRet {
    sbi_call_5(
        EID_RFNC,
        REMOTE_HFENCE_GVMA_VMID,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
        vmid,
    )
}

/// §8.5
#[inline]
pub fn remote_hfence_gvma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    sbi_call_4(
        EID_RFNC,
        REMOTE_HFENCE_GVMA,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
    )
}

/// §8.6
#[inline]
pub fn remote_hfence_vvma_asid(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
    asid: usize,
) -> SbiRet {
    sbi_call_5(
        EID_RFNC,
        REMOTE_HFENCE_VVMA_ASID,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
        asid,
    )
}

/// §8.7
#[inline]
pub fn remote_hfence_vvma(
    hart_mask: usize,
    hart_mask_base: usize,
    start_addr: usize,
    size: usize,
) -> SbiRet {
    sbi_call_4(
        EID_RFNC,
        REMOTE_HFENCE_VVMA,
        hart_mask,
        hart_mask_base,
        start_addr,
        size,
    )
}
