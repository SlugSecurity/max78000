#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x20 - Read to test and set, returns prior value. Write 0 to clear semaphore."]
    pub semaphores: [SEMAPHORES; 8],
    _reserved1: [u8; 0x20],
    #[doc = "0x40 - Semaphore IRQ0 register."]
    pub irq0: IRQ0,
    #[doc = "0x44 - Semaphore Mailbox 0 register."]
    pub mail0: MAIL0,
    #[doc = "0x48 - Semaphore IRQ1 register."]
    pub irq1: IRQ1,
    #[doc = "0x4c - Semaphore Mailbox 1 register."]
    pub mail1: MAIL1,
    _reserved5: [u8; 0xb0],
    #[doc = "0x100 - Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
    pub status: STATUS,
}
#[doc = "SEMAPHORES (rw) register accessor: Read to test and set, returns prior value. Write 0 to clear semaphore.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`semaphores::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`semaphores::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`semaphores`]
module"]
pub type SEMAPHORES = crate::Reg<semaphores::SEMAPHORES_SPEC>;
#[doc = "Read to test and set, returns prior value. Write 0 to clear semaphore."]
pub mod semaphores;
#[doc = "irq0 (rw) register accessor: Semaphore IRQ0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`irq0`]
module"]
pub type IRQ0 = crate::Reg<irq0::IRQ0_SPEC>;
#[doc = "Semaphore IRQ0 register."]
pub mod irq0;
#[doc = "mail0 (rw) register accessor: Semaphore Mailbox 0 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mail0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mail0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mail0`]
module"]
pub type MAIL0 = crate::Reg<mail0::MAIL0_SPEC>;
#[doc = "Semaphore Mailbox 0 register."]
pub mod mail0;
#[doc = "irq1 (rw) register accessor: Semaphore IRQ1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`irq1`]
module"]
pub type IRQ1 = crate::Reg<irq1::IRQ1_SPEC>;
#[doc = "Semaphore IRQ1 register."]
pub mod irq1;
#[doc = "mail1 (rw) register accessor: Semaphore Mailbox 1 register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mail1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mail1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mail1`]
module"]
pub type MAIL1 = crate::Reg<mail1::MAIL1_SPEC>;
#[doc = "Semaphore Mailbox 1 register."]
pub mod mail1;
#[doc = "status (rw) register accessor: Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`status`]
module"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Semaphore status bits. 0 indicates the semaphore is free, 1 indicates taken."]
pub mod status;
