#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - COMP Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - COMP Result Register"]
    pub result: crate::Reg<result::RESULT_SPEC>,
    #[doc = "0x08 - COMP Result Latch Register"]
    pub result_latch: crate::Reg<result_latch::RESULT_LATCH_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "COMP Configuration Register"]
pub mod cfg;
#[doc = "RESULT register accessor: an alias for `Reg<RESULT_SPEC>`"]
pub type RESULT = crate::Reg<result::RESULT_SPEC>;
#[doc = "COMP Result Register"]
pub mod result;
#[doc = "RESULT_LATCH register accessor: an alias for `Reg<RESULT_LATCH_SPEC>`"]
pub type RESULT_LATCH = crate::Reg<result_latch::RESULT_LATCH_SPEC>;
#[doc = "COMP Result Latch Register"]
pub mod result_latch;
