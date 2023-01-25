use crate::binary::{sbi_call_1, sbi_call_3, SbiRet};

// todo: change numbers into sbi_spec constants

/// Write bytes to the debug console from input memory.
#[inline]
pub unsafe fn console_write(num_bytes: usize, base_addr_lo: usize, base_addr_hi: usize) -> SbiRet {
    sbi_call_3(0x4442434E, 0, num_bytes, base_addr_lo, base_addr_hi)
}

/// Read bytes from the debug console into an output memory.
pub unsafe fn console_read(num_bytes: usize, base_addr_lo: usize, base_addr_hi: usize) -> SbiRet {
    sbi_call_3(0x4442434E, 1, num_bytes, base_addr_lo, base_addr_hi)
}

/// Write a single byte to the debug console.
#[inline]
pub fn console_write_byte(byte: u8) -> SbiRet {
    sbi_call_1(0x4442434E, 2, byte as usize)
}
