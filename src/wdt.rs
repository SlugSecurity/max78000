#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Timer Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Windowed Watchdog Timer Reset Register."]
    pub rst: RST,
    #[doc = "0x08 - Windowed Watchdog Timer Clock Select Register."]
    pub clksel: CLKSEL,
    #[doc = "0x0c - Windowed Watchdog Timer Count Register."]
    pub cnt: CNT,
}
#[doc = "CTRL (rw) register accessor: Watchdog Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Watchdog Timer Control Register."]
pub mod ctrl;
#[doc = "RST (w) register accessor: Windowed Watchdog Timer Reset Register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rst`]
module"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Windowed Watchdog Timer Reset Register."]
pub mod rst;
#[doc = "CLKSEL (rw) register accessor: Windowed Watchdog Timer Clock Select Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clksel`]
module"]
pub type CLKSEL = crate::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Windowed Watchdog Timer Clock Select Register."]
pub mod clksel;
#[doc = "CNT (r) register accessor: Windowed Watchdog Timer Count Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Windowed Watchdog Timer Count Register."]
pub mod cnt;
