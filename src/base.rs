//! Chapter 4. Base Extension (EID #0x10)

use crate::binary::{sbi_call_0, sbi_call_1};

pub use sbi_spec::base::*;

/// §4.1
#[inline]
pub fn get_spec_version() -> usize {
    sbi_call_0(EID_BASE, GET_SBI_IMPL_ID).value
}

/// §4.2
#[inline]
pub fn get_sbi_impl_id() -> usize {
    sbi_call_0(EID_BASE, GET_SBI_IMPL_ID).value
}

/// §4.3
#[inline]
pub fn get_sbi_impl_version() -> usize {
    sbi_call_0(EID_BASE, GET_SBI_IMPL_VERSION).value
}

/// §4.4
#[inline]
pub fn probe_extension(extension_id: usize) -> usize {
    sbi_call_1(EID_BASE, PROBE_EXTENSION, extension_id).value
}

/// §4.5
#[inline]
pub fn get_mvendorid() -> usize {
    sbi_call_0(EID_BASE, GET_MVENDORID).value
}

/// §4.6
#[inline]
pub fn get_marchid() -> usize {
    sbi_call_0(EID_BASE, GET_MARCHID).value
}

/// §4.7
#[inline]
pub fn get_mimpid() -> usize {
    sbi_call_0(EID_BASE, GET_MIMPID).value
}
