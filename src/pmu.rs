//! Chapter 11. Performance Monitoring Unit Extension (EID #0x504D55 "PMU")

use crate::binary::{sbi_call_0, sbi_call_1, sbi_call_3, SbiRet};

use sbi_spec::pmu::{
    EID_PMU, PMU_COUNTER_CONFIG_MATCHING, PMU_COUNTER_FW_READ, PMU_COUNTER_GET_INFO,
    PMU_COUNTER_START, PMU_COUNTER_STOP, PMU_NUM_COUNTERS,
};

/// §11.5
#[inline]
pub fn pmu_num_counters() -> SbiRet {
    sbi_call_0(EID_PMU, PMU_NUM_COUNTERS)
}

/// §11.6
#[inline]
pub fn pmu_counter_get_info(counter_idx: usize) -> SbiRet {
    sbi_call_1(EID_PMU, PMU_COUNTER_GET_INFO, counter_idx)
}

/// §11.7
#[inline]
pub fn pmu_counter_config_matching<T>(
    counter_idx_base: usize,
    counter_idx_mask: usize,
    config_flags: T,
    event_idx: usize,
    event_data: u64,
) -> SbiRet
where
    T: ConfigFlags,
{
    match () {
        #[cfg(target_pointer_width = "32")]
        () => crate::binary::sbi_call_6(
            EID_PMU,
            PMU_COUNTER_CONFIG_MATCHING,
            counter_idx_base,
            counter_idx_mask,
            config_flags.raw(),
            event_idx,
            event_data as _,
            (event_data >> 32) as _,
        ),
        #[cfg(target_pointer_width = "64")]
        () => crate::binary::sbi_call_5(
            EID_PMU,
            PMU_COUNTER_CONFIG_MATCHING,
            counter_idx_base,
            counter_idx_mask,
            config_flags.raw(),
            event_idx,
            event_data as _,
        ),
    }
}

/// §11.8
#[inline]
pub fn pmu_counter_start<T>(
    counter_idx_base: usize,
    counter_idx_mask: usize,
    start_flags: T,
    initial_value: u64,
) -> SbiRet
where
    T: StartFlags,
{
    match () {
        #[cfg(target_pointer_width = "32")]
        () => crate::binary::sbi_call_5(
            EID_PMU,
            PMU_COUNTER_START,
            counter_idx_base,
            counter_idx_mask,
            start_flags.raw(),
            initial_value as _,
            (initial_value >> 32) as _,
        ),
        #[cfg(target_pointer_width = "64")]
        () => crate::binary::sbi_call_4(
            EID_PMU,
            PMU_COUNTER_START,
            counter_idx_base,
            counter_idx_mask,
            start_flags.raw(),
            initial_value as _,
        ),
    }
}

/// §11.9
#[inline]
pub fn pmu_counter_stop<T>(
    counter_idx_base: usize,
    counter_idx_mask: usize,
    stop_flags: T,
) -> SbiRet
where
    T: StopFlags,
{
    sbi_call_3(
        EID_PMU,
        PMU_COUNTER_STOP,
        counter_idx_base,
        counter_idx_mask,
        stop_flags.raw(),
    )
}

/// §11.10
#[inline]
pub fn pmu_counter_fw_read(counter_idx: usize) -> SbiRet {
    sbi_call_1(EID_PMU, PMU_COUNTER_FW_READ, counter_idx)
}

/// Flags to configuate performance counter
pub trait ConfigFlags {
    fn raw(&self) -> usize;
}

impl ConfigFlags for usize {
    #[inline]
    fn raw(&self) -> usize {
        *self
    }
}

/// Flags to start performance counter
pub trait StartFlags {
    fn raw(&self) -> usize;
}

impl StartFlags for usize {
    #[inline]
    fn raw(&self) -> usize {
        *self
    }
}

/// Flags to stop performance counter
pub trait StopFlags {
    fn raw(&self) -> usize;
}

impl StopFlags for usize {
    #[inline]
    fn raw(&self) -> usize {
        *self
    }
}
