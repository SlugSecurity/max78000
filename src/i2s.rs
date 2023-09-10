#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global mode channel."]
    pub ctrl0ch0: CTRL0CH0,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Local channel Setup."]
    pub ctrl1ch0: CTRL1CH0,
    _reserved2: [u8; 0x0c],
    #[doc = "0x20 - Filter."]
    pub filtch0: FILTCH0,
    _reserved3: [u8; 0x0c],
    #[doc = "0x30 - DMA Control."]
    pub dmach0: DMACH0,
    _reserved4: [u8; 0x0c],
    #[doc = "0x40 - I2S Fifo."]
    pub fifoch0: FIFOCH0,
    _reserved5: [u8; 0x0c],
    #[doc = "0x50 - ISR Status."]
    pub intfl: INTFL,
    #[doc = "0x54 - Interrupt Enable."]
    pub inten: INTEN,
    #[doc = "0x58 - Ext Control."]
    pub extsetup: EXTSETUP,
    #[doc = "0x5c - Wakeup Enable."]
    pub wken: WKEN,
    #[doc = "0x60 - Wakeup Flags."]
    pub wkfl: WKFL,
}
#[doc = "CTRL0CH0 (rw) register accessor: Global mode channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl0ch0`]
module"]
pub type CTRL0CH0 = crate::Reg<ctrl0ch0::CTRL0CH0_SPEC>;
#[doc = "Global mode channel."]
pub mod ctrl0ch0;
#[doc = "CTRL1CH0 (rw) register accessor: Local channel Setup.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1ch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1ch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1ch0`]
module"]
pub type CTRL1CH0 = crate::Reg<ctrl1ch0::CTRL1CH0_SPEC>;
#[doc = "Local channel Setup."]
pub mod ctrl1ch0;
#[doc = "FILTCH0 (rw) register accessor: Filter.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`filtch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`filtch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`filtch0`]
module"]
pub type FILTCH0 = crate::Reg<filtch0::FILTCH0_SPEC>;
#[doc = "Filter."]
pub mod filtch0;
#[doc = "DMACH0 (rw) register accessor: DMA Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dmach0`]
module"]
pub type DMACH0 = crate::Reg<dmach0::DMACH0_SPEC>;
#[doc = "DMA Control."]
pub mod dmach0;
#[doc = "FIFOCH0 (rw) register accessor: I2S Fifo.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifoch0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifoch0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifoch0`]
module"]
pub type FIFOCH0 = crate::Reg<fifoch0::FIFOCH0_SPEC>;
#[doc = "I2S Fifo."]
pub mod fifoch0;
#[doc = "INTFL (rw) register accessor: ISR Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "ISR Status."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Interrupt Enable."]
pub mod inten;
#[doc = "EXTSETUP (rw) register accessor: Ext Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`extsetup::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`extsetup::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`extsetup`]
module"]
pub type EXTSETUP = crate::Reg<extsetup::EXTSETUP_SPEC>;
#[doc = "Ext Control."]
pub mod extsetup;
#[doc = "WKEN (rw) register accessor: Wakeup Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Wakeup Enable."]
pub mod wken;
#[doc = "WKFL (rw) register accessor: Wakeup Flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Wakeup Flags."]
pub mod wkfl;
