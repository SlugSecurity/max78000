#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Low Power Control Register."]
    pub lpcn: LPCN,
    #[doc = "0x04 - Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
    pub lpwkst0: LPWKST0,
    #[doc = "0x08 - Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
    pub lpwken0: LPWKEN0,
    #[doc = "0x0c - Low Power I/O Wakeup Status Register 1. This register indicates the low power wakeup status for GPIO1."]
    pub lpwkst1: LPWKST1,
    #[doc = "0x10 - Low Power I/O Wakeup Enable Register 1. This register enables low power wakeup functionality for GPIO1."]
    pub lpwken1: LPWKEN1,
    #[doc = "0x14 - Low Power I/O Wakeup Status Register 2. This register indicates the low power wakeup status for GPIO2."]
    pub lpwkst2: LPWKST2,
    #[doc = "0x18 - Low Power I/O Wakeup Enable Register 2. This register enables low power wakeup functionality for GPIO2."]
    pub lpwken2: LPWKEN2,
    #[doc = "0x1c - Low Power I/O Wakeup Status Register 3. This register indicates the low power wakeup status for GPIO3."]
    pub lpwkst3: LPWKST3,
    #[doc = "0x20 - Low Power I/O Wakeup Enable Register 3. This register enables low power wakeup functionality for GPIO3."]
    pub lpwken3: LPWKEN3,
    _reserved9: [u8; 0x0c],
    #[doc = "0x30 - Low Power Peripheral Wakeup Status Register."]
    pub lppwst: LPPWST,
    #[doc = "0x34 - Low Power Peripheral Wakeup Enable Register."]
    pub lppwen: LPPWEN,
    _reserved11: [u8; 0x10],
    #[doc = "0x48 - General Purpose Register 0"]
    pub gp0: GP0,
    #[doc = "0x4c - General Purpose Register 1"]
    pub gp1: GP1,
}
#[doc = "LPCN (rw) register accessor: Low Power Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpcn`]
module"]
pub type LPCN = crate::Reg<lpcn::LPCN_SPEC>;
#[doc = "Low Power Control Register."]
pub mod lpcn;
#[doc = "LPWKST0 (rw) register accessor: Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpwkst0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpwkst0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpwkst0`]
module"]
pub type LPWKST0 = crate::Reg<lpwkst0::LPWKST0_SPEC>;
#[doc = "Low Power I/O Wakeup Status Register 0. This register indicates the low power wakeup status for GPIO0."]
pub mod lpwkst0;
#[doc = "LPWKEN0 (rw) register accessor: Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpwken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpwken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lpwken0`]
module"]
pub type LPWKEN0 = crate::Reg<lpwken0::LPWKEN0_SPEC>;
#[doc = "Low Power I/O Wakeup Enable Register 0. This register enables low power wakeup functionality for GPIO0."]
pub mod lpwken0;
pub use lpwken0 as lpwken1;
pub use lpwken0 as lpwken2;
pub use lpwken0 as lpwken3;
pub use lpwkst0 as lpwkst1;
pub use lpwkst0 as lpwkst2;
pub use lpwkst0 as lpwkst3;
pub use LPWKEN0 as LPWKEN1;
pub use LPWKEN0 as LPWKEN2;
pub use LPWKEN0 as LPWKEN3;
pub use LPWKST0 as LPWKST1;
pub use LPWKST0 as LPWKST2;
pub use LPWKST0 as LPWKST3;
#[doc = "LPPWST (rw) register accessor: Low Power Peripheral Wakeup Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lppwst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lppwst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lppwst`]
module"]
pub type LPPWST = crate::Reg<lppwst::LPPWST_SPEC>;
#[doc = "Low Power Peripheral Wakeup Status Register."]
pub mod lppwst;
#[doc = "LPPWEN (rw) register accessor: Low Power Peripheral Wakeup Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lppwen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lppwen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`lppwen`]
module"]
pub type LPPWEN = crate::Reg<lppwen::LPPWEN_SPEC>;
#[doc = "Low Power Peripheral Wakeup Enable Register."]
pub mod lppwen;
#[doc = "GP0 (rw) register accessor: General Purpose Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gp0`]
module"]
pub type GP0 = crate::Reg<gp0::GP0_SPEC>;
#[doc = "General Purpose Register 0"]
pub mod gp0;
#[doc = "GP1 (rw) register accessor: General Purpose Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`gp1`]
module"]
pub type GP1 = crate::Reg<gp1::GP1_SPEC>;
#[doc = "General Purpose Register 1"]
pub mod gp1;
