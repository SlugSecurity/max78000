#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctl: CTL,
    #[doc = "0x04 - Status Fields"]
    pub stat: STAT,
    #[doc = "0x08 - Direct control of target voltage"]
    pub direct: DIRECT,
    #[doc = "0x0c - Monitor Delay"]
    pub mon: MON,
    #[doc = "0x10 - Up Delay Register"]
    pub adj_up: ADJ_UP,
    #[doc = "0x14 - Down Delay Register"]
    pub adj_dwn: ADJ_DWN,
    #[doc = "0x18 - Up Delay Register"]
    pub thres_cmp: THRES_CMP,
    #[doc = "0x1c..0x30 - DVS Tap Select Register"]
    pub tap_sel: [TAP_SEL; 5],
}
#[doc = "CTL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctl`]
module"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control Register"]
pub mod ctl;
#[doc = "STAT (rw) register accessor: Status Fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Fields"]
pub mod stat;
#[doc = "DIRECT (rw) register accessor: Direct control of target voltage\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`direct::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`direct::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`direct`]
module"]
pub type DIRECT = crate::Reg<direct::DIRECT_SPEC>;
#[doc = "Direct control of target voltage"]
pub mod direct;
#[doc = "MON (rw) register accessor: Monitor Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mon::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mon::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mon`]
module"]
pub type MON = crate::Reg<mon::MON_SPEC>;
#[doc = "Monitor Delay"]
pub mod mon;
#[doc = "ADJ_UP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adj_up::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adj_up::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adj_up`]
module"]
pub type ADJ_UP = crate::Reg<adj_up::ADJ_UP_SPEC>;
#[doc = "Up Delay Register"]
pub mod adj_up;
#[doc = "ADJ_DWN (rw) register accessor: Down Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adj_dwn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adj_dwn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`adj_dwn`]
module"]
pub type ADJ_DWN = crate::Reg<adj_dwn::ADJ_DWN_SPEC>;
#[doc = "Down Delay Register"]
pub mod adj_dwn;
#[doc = "THRES_CMP (rw) register accessor: Up Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`thres_cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`thres_cmp`]
module"]
pub type THRES_CMP = crate::Reg<thres_cmp::THRES_CMP_SPEC>;
#[doc = "Up Delay Register"]
pub mod thres_cmp;
#[doc = "TAP_SEL (rw) register accessor: DVS Tap Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tap_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tap_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`tap_sel`]
module"]
pub type TAP_SEL = crate::Reg<tap_sel::TAP_SEL_SPEC>;
#[doc = "DVS Tap Select Register"]
pub mod tap_sel;
