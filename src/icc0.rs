#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache ID Register."]
    pub info: INFO,
    #[doc = "0x04 - Memory Configuration Register."]
    pub sz: SZ,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - Cache Control and Status Register."]
    pub ctrl: CTRL,
    _reserved3: [u8; 0x05fc],
    #[doc = "0x700 - Invalidate All Registers."]
    pub invalidate: INVALIDATE,
}
#[doc = "INFO (r) register accessor: Cache ID Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`info`]
module"]
pub type INFO = crate::Reg<info::INFO_SPEC>;
#[doc = "Cache ID Register."]
pub mod info;
#[doc = "SZ (r) register accessor: Memory Configuration Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sz::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sz`]
module"]
pub type SZ = crate::Reg<sz::SZ_SPEC>;
#[doc = "Memory Configuration Register."]
pub mod sz;
#[doc = "CTRL (rw) register accessor: Cache Control and Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Cache Control and Status Register."]
pub mod ctrl;
#[doc = "INVALIDATE (rw) register accessor: Invalidate All Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`invalidate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`invalidate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`invalidate`]
module"]
pub type INVALIDATE = crate::Reg<invalidate::INVALIDATE_SPEC>;
#[doc = "Invalidate All Registers."]
pub mod invalidate;
