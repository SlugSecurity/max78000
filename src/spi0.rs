#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_fifo8: [u8; 0x04],
    #[doc = "0x04 - Register for controlling SPI peripheral."]
    pub ctrl0: CTRL0,
    #[doc = "0x08 - Register for controlling SPI peripheral."]
    pub ctrl1: CTRL1,
    #[doc = "0x0c - Register for controlling SPI peripheral."]
    pub ctrl2: CTRL2,
    #[doc = "0x10 - Register for controlling SPI peripheral/Slave Select Timing."]
    pub sstime: SSTIME,
    #[doc = "0x14 - Register for controlling SPI clock rate."]
    pub clkctrl: CLKCTRL,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c - Register for controlling DMA."]
    pub dma: DMA,
    #[doc = "0x20 - Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
    pub intfl: INTFL,
    #[doc = "0x24 - Register for enabling interrupts."]
    pub inten: INTEN,
    #[doc = "0x28 - Register for wake up flags. All bits in this register are write 1 to clear."]
    pub wkfl: WKFL,
    #[doc = "0x2c - Register for wake up enable."]
    pub wken: WKEN,
    #[doc = "0x30 - SPI Status register."]
    pub stat: STAT,
}
impl RegisterBlock {
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo8(&self) -> &[FIFO8; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo16(&self) -> &[FIFO16; 2] {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - Register for reading and writing the FIFO."]
    #[inline(always)]
    pub const fn fifo32(&self) -> &FIFO32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "FIFO32 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo32`]
module"]
pub type FIFO32 = crate::Reg<fifo32::FIFO32_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo32;
#[doc = "FIFO16 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo16`]
module"]
pub type FIFO16 = crate::Reg<fifo16::FIFO16_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo16;
#[doc = "FIFO8 (rw) register accessor: Register for reading and writing the FIFO.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo8`]
module"]
pub type FIFO8 = crate::Reg<fifo8::FIFO8_SPEC>;
#[doc = "Register for reading and writing the FIFO."]
pub mod fifo8;
#[doc = "CTRL0 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl0`]
module"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl1`]
module"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: Register for controlling SPI peripheral.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl2`]
module"]
pub type CTRL2 = crate::Reg<ctrl2::CTRL2_SPEC>;
#[doc = "Register for controlling SPI peripheral."]
pub mod ctrl2;
#[doc = "SSTIME (rw) register accessor: Register for controlling SPI peripheral/Slave Select Timing.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sstime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sstime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sstime`]
module"]
pub type SSTIME = crate::Reg<sstime::SSTIME_SPEC>;
#[doc = "Register for controlling SPI peripheral/Slave Select Timing."]
pub mod sstime;
#[doc = "CLKCTRL (rw) register accessor: Register for controlling SPI clock rate.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkctrl`]
module"]
pub type CLKCTRL = crate::Reg<clkctrl::CLKCTRL_SPEC>;
#[doc = "Register for controlling SPI clock rate."]
pub mod clkctrl;
#[doc = "DMA (rw) register accessor: Register for controlling DMA.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "Register for controlling DMA."]
pub mod dma;
#[doc = "INTFL (rw) register accessor: Register for reading and clearing interrupt flags. All bits are write 1 to clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "Register for reading and clearing interrupt flags. All bits are write 1 to clear."]
pub mod intfl;
#[doc = "INTEN (rw) register accessor: Register for enabling interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "Register for enabling interrupts."]
pub mod inten;
#[doc = "WKFL (rw) register accessor: Register for wake up flags. All bits in this register are write 1 to clear.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkfl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkfl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wkfl`]
module"]
pub type WKFL = crate::Reg<wkfl::WKFL_SPEC>;
#[doc = "Register for wake up flags. All bits in this register are write 1 to clear."]
pub mod wkfl;
#[doc = "WKEN (rw) register accessor: Register for wake up enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "Register for wake up enable."]
pub mod wken;
#[doc = "STAT (r) register accessor: SPI Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`stat`]
module"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status register."]
pub mod stat;
