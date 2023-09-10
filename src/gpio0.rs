#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
    pub en0: EN0,
    #[doc = "0x04 - GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
    pub en0_set: EN0_SET,
    #[doc = "0x08 - GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
    pub en0_clr: EN0_CLR,
    #[doc = "0x0c - GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
    pub outen: OUTEN,
    #[doc = "0x10 - GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
    pub outen_set: OUTEN_SET,
    #[doc = "0x14 - GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
    pub outen_clr: OUTEN_CLR,
    #[doc = "0x18 - GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
    pub out: OUT,
    #[doc = "0x1c - GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
    pub out_set: OUT_SET,
    #[doc = "0x20 - GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
    pub out_clr: OUT_CLR,
    #[doc = "0x24 - GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
    pub in_: IN,
    #[doc = "0x28 - GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
    pub intmode: INTMODE,
    #[doc = "0x2c - GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
    pub intpol: INTPOL,
    #[doc = "0x30 - GPIO Input Enable"]
    pub inen: INEN,
    #[doc = "0x34 - GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
    pub inten: INTEN,
    #[doc = "0x38 - GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
    pub inten_set: INTEN_SET,
    #[doc = "0x3c - GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
    pub inten_clr: INTEN_CLR,
    #[doc = "0x40 - GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
    pub intfl: INTFL,
    _reserved17: [u8; 0x04],
    #[doc = "0x48 - GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
    pub intfl_clr: INTFL_CLR,
    #[doc = "0x4c - GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
    pub wken: WKEN,
    #[doc = "0x50 - GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
    pub wken_set: WKEN_SET,
    #[doc = "0x54 - GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
    pub wken_clr: WKEN_CLR,
    _reserved21: [u8; 0x04],
    #[doc = "0x5c - GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
    pub dualedge: DUALEDGE,
    #[doc = "0x60 - GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub padctrl0: PADCTRL0,
    #[doc = "0x64 - GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
    pub padctrl1: PADCTRL1,
    #[doc = "0x68 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en1: EN1,
    #[doc = "0x6c - GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
    pub en1_set: EN1_SET,
    #[doc = "0x70 - GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
    pub en1_clr: EN1_CLR,
    #[doc = "0x74 - GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
    pub en2: EN2,
    #[doc = "0x78 - GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
    pub en2_set: EN2_SET,
    #[doc = "0x7c - GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
    pub en2_clr: EN2_CLR,
    _reserved30: [u8; 0x28],
    #[doc = "0xa8 - GPIO Input Hysteresis Enable."]
    pub hysen: HYSEN,
    #[doc = "0xac - GPIO Slew Rate Enable Register."]
    pub srsel: SRSEL,
    #[doc = "0xb0 - GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds0: DS0,
    #[doc = "0xb4 - GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
    pub ds1: DS1,
    #[doc = "0xb8 - GPIO Pull Select Mode."]
    pub ps: PS,
    _reserved35: [u8; 0x04],
    #[doc = "0xc0 - GPIO Voltage Select."]
    pub vssel: VSSEL,
}
#[doc = "EN0 (rw) register accessor: GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0`]
module"]
pub type EN0 = crate::Reg<en0::EN0_SPEC>;
#[doc = "GPIO Function Enable Register. Each bit controls the GPIO_EN setting for one GPIO pin on the associated port."]
pub mod en0;
#[doc = "EN0_SET (rw) register accessor: GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0_set`]
module"]
pub type EN0_SET = crate::Reg<en0_set::EN0_SET_SPEC>;
#[doc = "GPIO Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN to 1, without affecting other bits in that register."]
pub mod en0_set;
#[doc = "EN0_CLR (rw) register accessor: GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en0_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en0_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en0_clr`]
module"]
pub type EN0_CLR = crate::Reg<en0_clr::EN0_CLR_SPEC>;
#[doc = "GPIO Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN to 0, without affecting other bits in that register."]
pub mod en0_clr;
#[doc = "OUTEN (rw) register accessor: GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outen`]
module"]
pub type OUTEN = crate::Reg<outen::OUTEN_SPEC>;
#[doc = "GPIO Output Enable Register. Each bit controls the GPIO_OUT_EN setting for one GPIO pin in the associated port."]
pub mod outen;
#[doc = "OUTEN_SET (rw) register accessor: GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outen_set`]
module"]
pub type OUTEN_SET = crate::Reg<outen_set::OUTEN_SET_SPEC>;
#[doc = "GPIO Output Enable Set Function Enable Register. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT_EN to 1, without affecting other bits in that register."]
pub mod outen_set;
#[doc = "OUTEN_CLR (rw) register accessor: GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`outen_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`outen_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`outen_clr`]
module"]
pub type OUTEN_CLR = crate::Reg<outen_clr::OUTEN_CLR_SPEC>;
#[doc = "GPIO Output Enable Clear Function Enable Register. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT_EN to 0, without affecting other bits in that register."]
pub mod outen_clr;
#[doc = "OUT (rw) register accessor: GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out`]
module"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "GPIO Output Register. Each bit controls the GPIO_OUT setting for one pin in the associated port. This register can be written either directly, or by using the GPIO_OUT_SET and GPIO_OUT_CLR registers."]
pub mod out;
#[doc = "OUT_SET (w) register accessor: GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_set`]
module"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "GPIO Output Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_OUT to 1, without affecting other bits in that register."]
pub mod out_set;
#[doc = "OUT_CLR (w) register accessor: GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`out_clr`]
module"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "GPIO Output Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_OUT to 0, without affecting other bits in that register."]
pub mod out_clr;
#[doc = "IN (r) register accessor: GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`in_`]
module"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "GPIO Input Register. Read-only register to read from the logic states of the GPIO pins on this port."]
pub mod in_;
#[doc = "INTMODE (rw) register accessor: GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intmode`]
module"]
pub type INTMODE = crate::Reg<intmode::INTMODE_SPEC>;
#[doc = "GPIO Interrupt Mode Register. Each bit in this register controls the interrupt mode setting for the associated GPIO pin on this port."]
pub mod intmode;
#[doc = "INTPOL (rw) register accessor: GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intpol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intpol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intpol`]
module"]
pub type INTPOL = crate::Reg<intpol::INTPOL_SPEC>;
#[doc = "GPIO Interrupt Polarity Register. Each bit in this register controls the interrupt polarity setting for one GPIO pin in the associated port."]
pub mod intpol;
#[doc = "INEN (rw) register accessor: GPIO Input Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inen`]
module"]
pub type INEN = crate::Reg<inen::INEN_SPEC>;
#[doc = "GPIO Input Enable"]
pub mod inen;
#[doc = "INTEN (rw) register accessor: GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten`]
module"]
pub type INTEN = crate::Reg<inten::INTEN_SPEC>;
#[doc = "GPIO Interrupt Enable Register. Each bit in this register controls the GPIO interrupt enable for the associated pin on the GPIO port."]
pub mod inten;
#[doc = "INTEN_SET (rw) register accessor: GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten_set`]
module"]
pub type INTEN_SET = crate::Reg<inten_set::INTEN_SET_SPEC>;
#[doc = "GPIO Interrupt Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_INT_EN to 1, without affecting other bits in that register."]
pub mod inten_set;
#[doc = "INTEN_CLR (rw) register accessor: GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`inten_clr`]
module"]
pub type INTEN_CLR = crate::Reg<inten_clr::INTEN_CLR_SPEC>;
#[doc = "GPIO Interrupt Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_EN to 0, without affecting other bits in that register."]
pub mod inten_clr;
#[doc = "INTFL (r) register accessor: GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl`]
module"]
pub type INTFL = crate::Reg<intfl::INTFL_SPEC>;
#[doc = "GPIO Interrupt Status Register. Each bit in this register contains the pending interrupt status for the associated GPIO pin in this port."]
pub mod intfl;
#[doc = "INTFL_CLR (rw) register accessor: GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intfl_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intfl_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`intfl_clr`]
module"]
pub type INTFL_CLR = crate::Reg<intfl_clr::INTFL_CLR_SPEC>;
#[doc = "GPIO Status Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_INT_STAT to 0, without affecting other bits in that register."]
pub mod intfl_clr;
#[doc = "WKEN (rw) register accessor: GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wken`]
module"]
pub type WKEN = crate::Reg<wken::WKEN_SPEC>;
#[doc = "GPIO Wake Enable Register. Each bit in this register controls the PMU wakeup enable for the associated GPIO pin in this port."]
pub mod wken;
#[doc = "WKEN_SET (rw) register accessor: GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wken_set`]
module"]
pub type WKEN_SET = crate::Reg<wken_set::WKEN_SET_SPEC>;
#[doc = "GPIO Wake Enable Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_WAKE_EN to 1, without affecting other bits in that register."]
pub mod wken_set;
#[doc = "WKEN_CLR (rw) register accessor: GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wken_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wken_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`wken_clr`]
module"]
pub type WKEN_CLR = crate::Reg<wken_clr::WKEN_CLR_SPEC>;
#[doc = "GPIO Wake Enable Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_WAKE_EN to 0, without affecting other bits in that register."]
pub mod wken_clr;
#[doc = "DUALEDGE (rw) register accessor: GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dualedge::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dualedge::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`dualedge`]
module"]
pub type DUALEDGE = crate::Reg<dualedge::DUALEDGE_SPEC>;
#[doc = "GPIO Interrupt Dual Edge Mode Register. Each bit in this register selects dual edge mode for the associated GPIO pin in this port."]
pub mod dualedge;
#[doc = "PADCTRL0 (rw) register accessor: GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`padctrl0`]
module"]
pub type PADCTRL0 = crate::Reg<padctrl0::PADCTRL0_SPEC>;
#[doc = "GPIO Input Mode Config 1. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod padctrl0;
#[doc = "PADCTRL1 (rw) register accessor: GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`padctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`padctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`padctrl1`]
module"]
pub type PADCTRL1 = crate::Reg<padctrl1::PADCTRL1_SPEC>;
#[doc = "GPIO Input Mode Config 2. Each bit in this register enables the weak pull-up for the associated GPIO pin in this port."]
pub mod padctrl1;
#[doc = "EN1 (rw) register accessor: GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1`]
module"]
pub type EN1 = crate::Reg<en1::EN1_SPEC>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en1;
#[doc = "EN1_SET (rw) register accessor: GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en1_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1_set`]
module"]
pub type EN1_SET = crate::Reg<en1_set::EN1_SET_SPEC>;
#[doc = "GPIO Alternate Function Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN1 to 1, without affecting other bits in that register."]
pub mod en1_set;
#[doc = "EN1_CLR (rw) register accessor: GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en1_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en1_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en1_clr`]
module"]
pub type EN1_CLR = crate::Reg<en1_clr::EN1_CLR_SPEC>;
#[doc = "GPIO Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN1 to 0, without affecting other bits in that register."]
pub mod en1_clr;
#[doc = "EN2 (rw) register accessor: GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2`]
module"]
pub type EN2 = crate::Reg<en2::EN2_SPEC>;
#[doc = "GPIO Alternate Function Enable Register. Each bit in this register selects between primary/secondary functions for the associated GPIO pin in this port."]
pub mod en2;
#[doc = "EN2_SET (rw) register accessor: GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2_set`]
module"]
pub type EN2_SET = crate::Reg<en2_set::EN2_SET_SPEC>;
#[doc = "GPIO Alternate Function 2 Set. Writing a 1 to one or more bits in this register sets the bits in the same positions in GPIO_EN2 to 1, without affecting other bits in that register."]
pub mod en2_set;
#[doc = "EN2_CLR (rw) register accessor: GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en2_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en2_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`en2_clr`]
module"]
pub type EN2_CLR = crate::Reg<en2_clr::EN2_CLR_SPEC>;
#[doc = "GPIO Wake Alternate Function Clear. Writing a 1 to one or more bits in this register clears the bits in the same positions in GPIO_EN2 to 0, without affecting other bits in that register."]
pub mod en2_clr;
#[doc = "HYSEN (rw) register accessor: GPIO Input Hysteresis Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hysen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hysen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`hysen`]
module"]
pub type HYSEN = crate::Reg<hysen::HYSEN_SPEC>;
#[doc = "GPIO Input Hysteresis Enable."]
pub mod hysen;
#[doc = "SRSEL (rw) register accessor: GPIO Slew Rate Enable Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`srsel`]
module"]
pub type SRSEL = crate::Reg<srsel::SRSEL_SPEC>;
#[doc = "GPIO Slew Rate Enable Register."]
pub mod srsel;
#[doc = "DS0 (rw) register accessor: GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ds0`]
module"]
pub type DS0 = crate::Reg<ds0::DS0_SPEC>;
#[doc = "GPIO Drive Strength Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds0;
#[doc = "DS1 (rw) register accessor: GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ds1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ds1`]
module"]
pub type DS1 = crate::Reg<ds1::DS1_SPEC>;
#[doc = "GPIO Drive Strength 1 Register. Each bit in this register selects the drive strength for the associated GPIO pin in this port. Refer to the Datasheet for sink/source current of GPIO pins in each mode."]
pub mod ds1;
#[doc = "PS (rw) register accessor: GPIO Pull Select Mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ps::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ps::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`ps`]
module"]
pub type PS = crate::Reg<ps::PS_SPEC>;
#[doc = "GPIO Pull Select Mode."]
pub mod ps;
#[doc = "VSSEL (rw) register accessor: GPIO Voltage Select.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`vssel`]
module"]
pub type VSSEL = crate::Reg<vssel::VSSEL_SPEC>;
#[doc = "GPIO Voltage Select."]
pub mod vssel;
