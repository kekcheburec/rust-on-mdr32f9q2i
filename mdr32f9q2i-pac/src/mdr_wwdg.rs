#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WWDG Command Register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x04 - WWDG Configuration Register"]
    pub cfr: crate::Reg<cfr::CFR_SPEC>,
    #[doc = "0x08 - WWDG Status Register"]
    pub sr: crate::Reg<sr::SR_SPEC>,
}
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "WWDG Command Register"]
pub mod cr;
#[doc = "CFR register accessor: an alias for `Reg<CFR_SPEC>`"]
pub type CFR = crate::Reg<cfr::CFR_SPEC>;
#[doc = "WWDG Configuration Register"]
pub mod cfr;
#[doc = "SR register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "WWDG Status Register"]
pub mod sr;
