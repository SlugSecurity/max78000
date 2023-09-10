#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register0."]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register."]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Status Register."]
    pub intfl0: INTFL0,
    #[doc = "0x0c - Interrupt Enable Register."]
    pub inten0: INTEN0,
    #[doc = "0x10 - Interrupt Status Register 1."]
    pub intfl1: INTFL1,
    #[doc = "0x14 - Interrupt Staus Register 1."]
    pub inten1: INTEN1,
    #[doc = "0x18 - FIFO Configuration Register."]
    pub fifolen: FIFOLEN,
    #[doc = "0x1c - Receive Control Register 0."]
    pub rxctrl0: RXCTRL0,
    #[doc = "0x20 - Receive Control Register 1."]
    pub rxctrl1: RXCTRL1,
    #[doc = "0x24 - Transmit Control Register 0."]
    pub txctrl0: TXCTRL0,
    #[doc = "0x28 - Transmit Control Register 1."]
    pub txctrl1: TXCTRL1,
    #[doc = "0x2c - Data Register."]
    pub fifo: FIFO,
    #[doc = "0x30 - Master Control Register."]
    pub mstctrl: MSTCTRL,
    #[doc = "0x34 - Clock Low Register."]
    pub clklo: CLKLO,
    #[doc = "0x38 - Clock high Register."]
    pub clkhi: CLKHI,
    #[doc = "0x3c - Clock high Register."]
    pub hsclk: HSCLK,
    #[doc = "0x40 - Timeout Register"]
    pub timeout: TIMEOUT,
    _reserved17: [u8; 0x04],
    #[doc = "0x48 - DMA Register."]
    pub dma: DMA,
    _reserved_18_slave0: [u8; 0x10],
}
impl RegisterBlock {
    #[doc = "0x4c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave0(&self) -> &SLAVE0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x4c..0x5c - Slave Address Register."]
    #[inline(always)]
    pub const fn slave_multi(&self) -> &[SLAVE_MULTI; 4] {
        unsafe { &*(self as *const Self).cast::<u8>().add(76usize).cast() }
    }
    #[doc = "0x50 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave1(&self) -> &SLAVE1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80usize).cast() }
    }
    #[doc = "0x54 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave2(&self) -> &SLAVE2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84usize).cast() }
    }
    #[doc = "0x58 - Slave Address Register."]
    #[inline(always)]
    pub const fn slave3(&self) -> &SLAVE3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88usize).cast() }
    }
}
#[doc = "CTRL (rw) register accessor: Control Register0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register0."]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register."]
pub mod status;
#[doc = "INTFL0 (rw) register accessor: Interrupt Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl0`]
module"]
pub type INTFL0 = crate::Reg<intfl0::INTFL0_SPEC>;
#[doc = "Interrupt Status Register."]
pub mod intfl0;
#[doc = "INTEN0 (rw) register accessor: Interrupt Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten0`]
module"]
pub type INTEN0 = crate::Reg<inten0::INTEN0_SPEC>;
#[doc = "Interrupt Enable Register."]
pub mod inten0;
#[doc = "INTFL1 (rw) register accessor: Interrupt Status Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl1`]
module"]
pub type INTFL1 = crate::Reg<intfl1::INTFL1_SPEC>;
#[doc = "Interrupt Status Register 1."]
pub mod intfl1;
#[doc = "INTEN1 (rw) register accessor: Interrupt Staus Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten1`]
module"]
pub type INTEN1 = crate::Reg<inten1::INTEN1_SPEC>;
#[doc = "Interrupt Staus Register 1."]
pub mod inten1;
#[doc = "FIFOLEN (rw) register accessor: FIFO Configuration Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifolen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifolen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifolen`]
module"]
pub type FIFOLEN = crate::Reg<fifolen::FIFOLEN_SPEC>;
#[doc = "FIFO Configuration Register."]
pub mod fifolen;
#[doc = "RXCTRL0 (rw) register accessor: Receive Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxctrl0`]
module"]
pub type RXCTRL0 = crate::Reg<rxctrl0::RXCTRL0_SPEC>;
#[doc = "Receive Control Register 0."]
pub mod rxctrl0;
#[doc = "RXCTRL1 (rw) register accessor: Receive Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`rxctrl1`]
module"]
pub type RXCTRL1 = crate::Reg<rxctrl1::RXCTRL1_SPEC>;
#[doc = "Receive Control Register 1."]
pub mod rxctrl1;
#[doc = "TXCTRL0 (rw) register accessor: Transmit Control Register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txctrl0`]
module"]
pub type TXCTRL0 = crate::Reg<txctrl0::TXCTRL0_SPEC>;
#[doc = "Transmit Control Register 0."]
pub mod txctrl0;
#[doc = "TXCTRL1 (rw) register accessor: Transmit Control Register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`txctrl1`]
module"]
pub type TXCTRL1 = crate::Reg<txctrl1::TXCTRL1_SPEC>;
#[doc = "Transmit Control Register 1."]
pub mod txctrl1;
#[doc = "FIFO (rw) register accessor: Data Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fifo`]
module"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "Data Register."]
pub mod fifo;
#[doc = "MSTCTRL (rw) register accessor: Master Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mstctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mstctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mstctrl`]
module"]
pub type MSTCTRL = crate::Reg<mstctrl::MSTCTRL_SPEC>;
#[doc = "Master Control Register."]
pub mod mstctrl;
#[doc = "CLKLO (rw) register accessor: Clock Low Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clklo`]
module"]
pub type CLKLO = crate::Reg<clklo::CLKLO_SPEC>;
#[doc = "Clock Low Register."]
pub mod clklo;
#[doc = "CLKHI (rw) register accessor: Clock high Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkhi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkhi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`clkhi`]
module"]
pub type CLKHI = crate::Reg<clkhi::CLKHI_SPEC>;
#[doc = "Clock high Register."]
pub mod clkhi;
#[doc = "HSCLK (rw) register accessor: Clock high Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hsclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hsclk`]
module"]
pub type HSCLK = crate::Reg<hsclk::HSCLK_SPEC>;
#[doc = "Clock high Register."]
pub mod hsclk;
#[doc = "TIMEOUT (rw) register accessor: Timeout Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`timeout`]
module"]
pub type TIMEOUT = crate::Reg<timeout::TIMEOUT_SPEC>;
#[doc = "Timeout Register"]
pub mod timeout;
#[doc = "DMA (rw) register accessor: DMA Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dma`]
module"]
pub type DMA = crate::Reg<dma::DMA_SPEC>;
#[doc = "DMA Register."]
pub mod dma;
#[doc = "SLAVE_MULTI (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave_multi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave_multi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave_multi`]
module"]
pub type SLAVE_MULTI = crate::Reg<slave_multi::SLAVE_MULTI_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave_multi;
#[doc = "SLAVE0 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave0`]
module"]
pub type SLAVE0 = crate::Reg<slave0::SLAVE0_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave0;
#[doc = "SLAVE1 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave1`]
module"]
pub type SLAVE1 = crate::Reg<slave1::SLAVE1_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave1;
#[doc = "SLAVE2 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave2`]
module"]
pub type SLAVE2 = crate::Reg<slave2::SLAVE2_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave2;
#[doc = "SLAVE3 (rw) register accessor: Slave Address Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slave3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`slave3`]
module"]
pub type SLAVE3 = crate::Reg<slave3::SLAVE3_SPEC>;
#[doc = "Slave Address Register."]
pub mod slave3;
