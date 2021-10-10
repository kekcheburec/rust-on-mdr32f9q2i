#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IWDG Key Register"]
    pub kr: crate::Reg<kr::KR_SPEC>,
    #[doc = "0x04 - IWDG Clock Prescaler Register"]
    pub pr: crate::Reg<pr::PR_SPEC>,
    #[doc = "0x08 - IWDG Reload Register"]
    pub rlr: crate::Reg<rlr::RLR_SPEC>,
    #[doc = "0x0c - IWDG Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "KR register accessor: an alias for `Reg<KR_SPEC>`"]
pub type KR = crate::Reg<kr::KR_SPEC>;
#[doc = "IWDG Key Register"]
pub mod kr;
#[doc = "PR register accessor: an alias for `Reg<PR_SPEC>`"]
pub type PR = crate::Reg<pr::PR_SPEC>;
#[doc = "IWDG Clock Prescaler Register"]
pub mod pr;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "IWDG Reload Register"]
pub mod rlr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "IWDG Status Register"]
pub mod sr;
