#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Hardware Version."]
    pub ver: VER,
    #[doc = "0x04 - FIFO Depth."]
    pub fifo_size: FIFO_SIZE,
    #[doc = "0x08 - Control Register."]
    pub ctrl: CTRL,
    #[doc = "0x0c - Interupt Enable Register."]
    pub int_en: INT_EN,
    #[doc = "0x10 - Interupt Flag Register."]
    pub int_fl: INT_FL,
    #[doc = "0x14 - DS Timing Code Register."]
    pub ds_timing_codes: DS_TIMING_CODES,
    _reserved6: [u8; 0x18],
    #[doc = "0x30 - FIFO DATA Register."]
    pub fifo_data: FIFO_DATA,
}
#[doc = "VER (rw) register accessor: Hardware Version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ver::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ver::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ver`]
module"]
pub type VER = crate::Reg<ver::VER_SPEC>;
#[doc = "Hardware Version."]
pub mod ver;
#[doc = "FIFO_SIZE (rw) register accessor: FIFO Depth.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo_size`]
module"]
pub type FIFO_SIZE = crate::Reg<fifo_size::FIFO_SIZE_SPEC>;
#[doc = "FIFO Depth."]
pub mod fifo_size;
#[doc = "CTRL (rw) register accessor: Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register."]
pub mod ctrl;
#[doc = "INT_EN (rw) register accessor: Interupt Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_en`]
module"]
pub type INT_EN = crate::Reg<int_en::INT_EN_SPEC>;
#[doc = "Interupt Enable Register."]
pub mod int_en;
#[doc = "INT_FL (rw) register accessor: Interupt Flag Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_fl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_fl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`int_fl`]
module"]
pub type INT_FL = crate::Reg<int_fl::INT_FL_SPEC>;
#[doc = "Interupt Flag Register."]
pub mod int_fl;
#[doc = "DS_TIMING_CODES (rw) register accessor: DS Timing Code Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds_timing_codes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds_timing_codes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ds_timing_codes`]
module"]
pub type DS_TIMING_CODES = crate::Reg<ds_timing_codes::DS_TIMING_CODES_SPEC>;
#[doc = "DS Timing Code Register."]
pub mod ds_timing_codes;
#[doc = "FIFO_DATA (rw) register accessor: FIFO DATA Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo_data`]
module"]
pub type FIFO_DATA = crate::Reg<fifo_data::FIFO_DATA_SPEC>;
#[doc = "FIFO DATA Register."]
pub mod fifo_data;
