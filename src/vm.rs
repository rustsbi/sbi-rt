/// A virtual memory algorithm
pub trait AddressSpace {
    /// Iterator of physical parts on this address space system
    type Window<P>: Iterator<Item = P>;
    /// Address translation errors, e.g. page faults
    type Error;
    /// Maps reference of a virtual object into a set of physical parts
    fn map<P: Virtual>(&self, virt_ref: P) -> Result<Self::Window<Part<P>>, Self::Error>;
}

/// A virtual memory object
pub trait Virtual {
    /// Returns virtual address of a memory object
    fn virtual_address(&self) -> usize;
    /// Returns length of a memory object
    fn virtual_len(&self) -> usize;
}

impl<'a, T> Virtual for &'a [T] {
    fn virtual_address(&self) -> usize {
        self.as_ptr() as usize
    }
    fn virtual_len(&self) -> usize {
        self.len()
    }
}

impl<'a, T> Virtual for &'a mut [T] {
    fn virtual_address(&self) -> usize {
        self.as_ptr() as usize
    }
    fn virtual_len(&self) -> usize {
        self.len()
    }
}

impl<'a> Virtual for &'a str {
    fn virtual_address(&self) -> usize {
        self.as_ptr() as usize
    }
    fn virtual_len(&self) -> usize {
        self.len()
    }
}

/// Reference of a part of a memory object
#[derive(Clone, Copy)]
pub struct Part<P> {
    len: usize,
    phys_addr_lo: usize,
    phys_addr_hi: usize,
    _marker: core::marker::PhantomData<P>,
}

impl<P> Part<P> {
    /// Create a memory object part by length and physical address
    pub const fn new(len: usize, phys_addrs: [usize; 2]) -> Self {
        Self {
            len,
            phys_addr_hi: phys_addrs[1],
            phys_addr_lo: phys_addrs[0],
            _marker: core::marker::PhantomData,
        }
    }
    /// Returns length of a memory object
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Returns physical address of a memory object by high part and low part
    pub const fn phys_addrs(&self) -> [usize; 2] {
        [self.phys_addr_hi, self.phys_addr_lo]
    }
}

/// Identical memory translation
pub struct Identical;

impl AddressSpace for Identical {
    type Window<P> = core::iter::Once<P>;
    type Error = core::convert::Infallible;
    fn map<P: Virtual>(&self, virt_ref: P) -> Result<Self::Window<Part<P>>, Self::Error> {
        let identical_ref = Part::new(virt_ref.virtual_len(), [0, virt_ref.virtual_address()]);
        Ok(core::iter::once(identical_ref))
    }
}
