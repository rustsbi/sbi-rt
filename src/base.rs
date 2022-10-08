//! Chapter 4. Base Extension (EID #0x10)

use crate::binary::{sbi_call_0, sbi_call_1};

use sbi_spec::base::{
    SbiSpecVersion, EID_BASE, GET_MARCHID, GET_MIMPID, GET_MVENDORID, GET_SBI_IMPL_ID,
    GET_SBI_IMPL_VERSION, GET_SBI_SPEC_VERSION, PROBE_EXTENSION,
};

/// §4.1
#[inline]
pub fn get_spec_version() -> SbiSpecVersion {
    SbiSpecVersion(sbi_call_0(EID_BASE, GET_SBI_SPEC_VERSION).value)
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

/// Probes information abort one SBI extension from current environment.
///
/// Returns 0 if given SBI `extension_id` is not available, or typically
/// 1 if it's available. Implementation would define further non-zero
/// return values for information about this extension if it is available.
///
/// This function is defined in RISC-V SBI Specification chapter 4.4.
/// According to introduction of chapter 4, all base extension functions
/// must success and return no error code.
#[inline]
pub fn probe_extension<E>(extension: E) -> ExtensionInfo
where
    E: Extension,
{
    let ans = sbi_call_1(EID_BASE, PROBE_EXTENSION, extension.extension_id());
    ExtensionInfo { raw: ans.value }
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

/// An SBI extension
pub trait Extension {
    /// Get a raw `extension_id` value to pass to SBI environment
    fn extension_id(&self) -> usize;
}

macro_rules! define_extension {
    ($($struct:ident($value:expr) #[$doc:meta])*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[$doc]
            pub struct $struct;
            impl Extension for $struct {
                #[inline]
                fn extension_id(&self) -> usize {
                    $value
                }
            }
        )*
    };
}

define_extension! {
    Base(sbi_spec::base::EID_BASE) /// Base extension
    Timer(sbi_spec::time::EID_TIME) /// Timer extension
    Ipi(sbi_spec::spi::EID_SPI) /// Inter-processor Interrupt extension
    Fence(sbi_spec::rfnc::EID_RFNC) /// Remote Fence extension
    Hsm(sbi_spec::hsm::EID_HSM) /// Hart State Monitor extension
    Reset(sbi_spec::srst::EID_SRST) /// System Reset extension
    Pmu(sbi_spec::pmu::EID_PMU) /// Performance Monitoring Unit extension
}

impl Extension for usize {
    #[inline]
    fn extension_id(&self) -> usize {
        *self
    }
}

impl Extension for isize {
    #[inline]
    fn extension_id(&self) -> usize {
        usize::from_ne_bytes(isize::to_ne_bytes(*self))
    }
}

/// Information about an SBI extension
#[derive(Clone, Copy, Debug)]
pub struct ExtensionInfo {
    pub raw: usize,
}

impl ExtensionInfo {
    /// Is this extension not available?
    #[inline]
    pub const fn is_unavailable(&self) -> bool {
        self.raw == 0
    }
}
