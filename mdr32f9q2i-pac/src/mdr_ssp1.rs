#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SSP Control0 Register"]
    pub cr0: crate::Reg<cr0::CR0_SPEC>,
    #[doc = "0x04 - SSP Control1 Register"]
    pub cr1: crate::Reg<cr1::CR1_SPEC>,
    #[doc = "0x08 - SSP Data Register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x0c - SSP Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
    #[doc = "0x10 - SSP Clock Prescaler Register"]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x14 - SSP Interrupt Mask Register"]
    pub imsc: crate::Reg<imsc::IMSC_SPEC>,
    #[doc = "0x18 - SSP Interrupt Pending Register"]
    pub ris: crate::Reg<ris::RIS_SPEC>,
    #[doc = "0x1c - SSP Masked Interrupt Pending Register"]
    pub mis: crate::Reg<mis::MIS_SPEC>,
    #[doc = "0x20 - SSP Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x24 - SSP DMA Control Register"]
    pub dmacr: crate::Reg<dmacr::DMACR_SPEC>,
}
#[doc = "CR0 register accessor: an alias for `Reg<CR0_SPEC>`"]
pub type CR0 = crate::Reg<cr0::CR0_SPEC>;
#[doc = "SSP Control0 Register"]
pub mod cr0;
#[doc = "CR1 register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "SSP Control1 Register"]
pub mod cr1;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "SSP Data Register"]
pub mod dr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "SSP Status Register"]
pub mod sr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "SSP Clock Prescaler Register"]
pub mod cpsr;
#[doc = "IMSC register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "SSP Interrupt Mask Register"]
pub mod imsc;
#[doc = "RIS register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "SSP Interrupt Pending Register"]
pub mod ris;
#[doc = "MIS register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "SSP Masked Interrupt Pending Register"]
pub mod mis;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "SSP Interrupt Clear Register"]
pub mod icr;
#[doc = "DMACR register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "SSP DMA Control Register"]
pub mod dmacr;
