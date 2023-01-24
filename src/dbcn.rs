use crate::binary::{sbi_call_1, sbi_call_3, SbiRet};
use crate::vm::AddressSpace;

// todo: change numbers into sbi_spec constants

/// Error occurred in debug console
pub enum DebugError<E> {
    Translation(E),
    Sbi(sbi_spec::binary::Error),
}

/// Write bytes to the debug console from input memory.
pub fn console_write<A>(addr_space: A, buf: &[u8]) -> Result<(), DebugError<A::Error>>
where
    A: AddressSpace,
{
    let parts = match addr_space.map(buf) {
        Ok(parts) => parts,
        Err(e) => return Err(DebugError::Translation(e)),
    };
    for part in parts {
        let [base_addr_hi, base_addr_lo] = part.phys_addrs();
        let sbiret = sbi_call_3(0x4442434E, 0, part.len(), base_addr_lo, base_addr_hi);
        match sbiret.into_result() {
            Ok(_) => continue,
            Err(e) => return Err(DebugError::Sbi(e)),
        }
    }
    Ok(())
}

/// Read bytes from the debug console into an output memory.
pub fn console_read<A>(addr_space: A, buf: &mut [u8]) -> Result<(), DebugError<A::Error>>
where
    A: AddressSpace,
{
    let parts = match addr_space.map(buf) {
        Ok(parts) => parts,
        Err(e) => return Err(DebugError::Translation(e)),
    };
    for part in parts {
        let [base_addr_hi, base_addr_lo] = part.phys_addrs();
        let sbiret = sbi_call_3(0x4442434E, 1, part.len(), base_addr_lo, base_addr_hi);
        match sbiret.into_result() {
            Ok(_) => continue,
            Err(e) => return Err(DebugError::Sbi(e)),
        }
    }
    Ok(())
}

/// Write a single byte to the debug console.
#[inline]
pub fn console_write_byte(byte: u8) -> SbiRet {
    sbi_call_1(0x4442434E, 2, byte as usize)
}
