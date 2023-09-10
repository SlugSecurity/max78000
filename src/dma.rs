#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register."]
    pub inten: INTEN,
    #[doc = "0x04 - DMA Interrupt Register."]
    pub intfl: INTFL,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100..0x180 - DMA Channel registers."]
    pub ch: [CH; 4],
}
#[doc = "INTEN (rw) register accessor: DMA Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "DMA Control Register."]
pub mod inten;
#[doc = "INTFL (r) register accessor: DMA Interrupt Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "DMA Interrupt Register."]
pub mod intfl;
#[doc = "DMA Channel registers."]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "DMA Channel registers."]
pub mod ch;
