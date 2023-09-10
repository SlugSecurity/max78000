#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Count. This register stores the current timer count."]
    pub cnt: CNT,
    #[doc = "0x04 - Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
    pub cmp: CMP,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
    pub intr: INTR,
    #[doc = "0x10 - Timer Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x14 - Timer Non-Overlapping Compare Register."]
    pub nolcmp: NOLCMP,
    #[doc = "0x18 - Preset register."]
    pub preset: PRESET,
    #[doc = "0x1c - Reload register."]
    pub reload: RELOAD,
    #[doc = "0x20 - Snapshot register."]
    pub snapshot: SNAPSHOT,
}
#[doc = "CNT (rw) register accessor: Count. This register stores the current timer count.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cnt`]
module"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Count. This register stores the current timer count."]
pub mod cnt;
#[doc = "CMP (rw) register accessor: Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`cmp`]
module"]
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
#[doc = "Compare. This register stores the compare value, which is used to set the maximum count value to initiate a reload of the timer to 0x0001."]
pub mod cmp;
#[doc = "INTR (rw) register accessor: Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intr`]
module"]
pub type INTR = crate::Reg<intr::INTR_SPEC>;
#[doc = "Clear Interrupt. Writing a value (0 or 1) to a bit in this register clears the associated interrupt."]
pub mod intr;
#[doc = "CTRL (rw) register accessor: Timer Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Timer Control Register."]
pub mod ctrl;
#[doc = "NOLCMP (rw) register accessor: Timer Non-Overlapping Compare Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nolcmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nolcmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`nolcmp`]
module"]
pub type NOLCMP = crate::Reg<nolcmp::NOLCMP_SPEC>;
#[doc = "Timer Non-Overlapping Compare Register."]
pub mod nolcmp;
#[doc = "PRESET (rw) register accessor: Preset register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`preset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`preset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`preset`]
module"]
pub type PRESET = crate::Reg<preset::PRESET_SPEC>;
#[doc = "Preset register."]
pub mod preset;
#[doc = "RELOAD (rw) register accessor: Reload register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`reload`]
module"]
pub type RELOAD = crate::Reg<reload::RELOAD_SPEC>;
#[doc = "Reload register."]
pub mod reload;
#[doc = "SNAPSHOT (rw) register accessor: Snapshot register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`snapshot::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`snapshot::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`snapshot`]
module"]
pub type SNAPSHOT = crate::Reg<snapshot::SNAPSHOT_SPEC>;
#[doc = "Snapshot register."]
pub mod snapshot;
