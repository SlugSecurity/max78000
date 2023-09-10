#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - 1-Wire Master Configuration."]
    pub cfg: CFG,
    #[doc = "0x04 - 1-Wire Master Clock Divisor."]
    pub clk_div_1us: CLK_DIV_1US,
    #[doc = "0x08 - 1-Wire Master Control/Status."]
    pub ctrl_stat: CTRL_STAT,
    #[doc = "0x0c - 1-Wire Master Data Buffer."]
    pub data: DATA,
    #[doc = "0x10 - 1-Wire Master Interrupt Flags."]
    pub intfl: INTFL,
    #[doc = "0x14 - 1-Wire Master Interrupt Enables."]
    pub inten: INTEN,
}
#[doc = "CFG (rw) register accessor: 1-Wire Master Configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "1-Wire Master Configuration."]
pub mod cfg;
#[doc = "CLK_DIV_1US (rw) register accessor: 1-Wire Master Clock Divisor.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_div_1us::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_div_1us::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clk_div_1us`]
module"]
pub type CLK_DIV_1US = crate::Reg<clk_div_1us::CLK_DIV_1US_SPEC>;
#[doc = "1-Wire Master Clock Divisor."]
pub mod clk_div_1us;
#[doc = "CTRL_STAT (rw) register accessor: 1-Wire Master Control/Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl_stat`]
module"]
pub type CTRL_STAT = crate::Reg<ctrl_stat::CTRL_STAT_SPEC>;
#[doc = "1-Wire Master Control/Status."]
pub mod ctrl_stat;
#[doc = "DATA (rw) register accessor: 1-Wire Master Data Buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`data`]
module"]
pub type DATA = crate::Reg<data::DATA_SPEC>;
#[doc = "1-Wire Master Data Buffer."]
pub mod data;
#[doc = "INTFL (rw) register accessor: 1-Wire Master Interrupt Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "1-Wire Master Interrupt Flags."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: 1-Wire Master Interrupt Enables.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "1-Wire Master Interrupt Enables."]
pub mod inten;
