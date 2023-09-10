#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC Control"]
    pub ctrl: CTRL,
    _reserved_1_datain8: [u8; 0x04],
    #[doc = "0x08 - CRC Polynomial"]
    pub poly: POLY,
    #[doc = "0x0c - Current CRC Value"]
    pub val: VAL,
}
impl RegisterBlock {
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain8(&self) -> &[DATAIN8; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain16(&self) -> &[DATAIN16; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - CRC Data Input"]
    #[inline(always)]
    pub const fn datain32(&self) -> &DATAIN32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: CRC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "CRC Control"]
pub mod ctrl;
#[doc = "DATAIN32 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datain32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datain32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datain32`]
module"]
pub type DATAIN32 = crate::Reg<datain32::DATAIN32_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain32;
#[doc = "DATAIN16 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datain16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datain16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datain16`]
module"]
pub type DATAIN16 = crate::Reg<datain16::DATAIN16_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain16;
#[doc = "DATAIN8 (rw) register accessor: CRC Data Input\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`datain8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`datain8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`datain8`]
module"]
pub type DATAIN8 = crate::Reg<datain8::DATAIN8_SPEC>;
#[doc = "CRC Data Input"]
pub mod datain8;
#[doc = "POLY (rw) register accessor: CRC Polynomial\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`poly`]
module"]
pub type POLY = crate::Reg<poly::POLY_SPEC>;
#[doc = "CRC Polynomial"]
pub mod poly;
#[doc = "VAL (rw) register accessor: Current CRC Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`val::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`val::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`val`]
module"]
pub type VAL = crate::Reg<val::VAL_SPEC>;
#[doc = "Current CRC Value"]
pub mod val;
