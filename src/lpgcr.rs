#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Low Power Reset Register."]
    pub rst: RST,
    #[doc = "0x0c - Low Power Peripheral Clock Disable Register."]
    pub pclkdis: PCLKDIS,
}
#[doc = "RST (rw) register accessor: Low Power Reset Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst`]
module"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Low Power Reset Register."]
pub mod rst;
#[doc = "PCLKDIS (rw) register accessor: Low Power Peripheral Clock Disable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pclkdis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pclkdis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`pclkdis`]
module"]
pub type PCLKDIS = crate::Reg<pclkdis::PCLKDIS_SPEC>;
#[doc = "Low Power Peripheral Clock Disable Register."]
pub mod pclkdis;
