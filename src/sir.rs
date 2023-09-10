#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Initialization Status Register."]
    pub sistat: SISTAT,
    #[doc = "0x04 - Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
    pub addr: ADDR,
    _reserved2: [u8; 0xf8],
    #[doc = "0x100 - funcstat register."]
    pub fstat: FSTAT,
    #[doc = "0x104 - Security function status register."]
    pub sfstat: SFSTAT,
}
#[doc = "SISTAT (r) register accessor: System Initialization Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sistat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sistat`]
module"]
pub type SISTAT = crate::Reg<sistat::SISTAT_SPEC>;
#[doc = "System Initialization Status Register."]
pub mod sistat;
#[doc = "ADDR (r) register accessor: Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`addr`]
module"]
pub type ADDR = crate::Reg<addr::ADDR_SPEC>;
#[doc = "Read-only field set by the SIB block if a CRC error occurs during the read of the OTP memory. Contains the failing address in OTP memory (when CRCERR equals 1)."]
pub mod addr;
#[doc = "FSTAT (r) register accessor: funcstat register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`fstat`]
module"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "funcstat register."]
pub mod fstat;
#[doc = "SFSTAT (r) register accessor: Security function status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfstat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`sfstat`]
module"]
pub type SFSTAT = crate::Reg<sfstat::SFSTAT_SPEC>;
#[doc = "Security function status register."]
pub mod sfstat;
