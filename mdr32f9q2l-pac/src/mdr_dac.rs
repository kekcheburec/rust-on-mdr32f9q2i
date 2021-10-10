#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DAC Configuration Register"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x04 - DAC1 Data Register"]
    pub dac1_data: crate::Reg<dac1_data::DAC1_DATA_SPEC>,
    #[doc = "0x08 - DAC2 Data Register"]
    pub dac2_data: crate::Reg<dac2_data::DAC2_DATA_SPEC>,
}
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "DAC Configuration Register"]
pub mod cfg;
#[doc = "DAC1_DATA register accessor: an alias for `Reg<DAC1_DATA_SPEC>`"]
pub type DAC1_DATA = crate::Reg<dac1_data::DAC1_DATA_SPEC>;
#[doc = "DAC1 Data Register"]
pub mod dac1_data;
#[doc = "DAC2_DATA register accessor: an alias for `Reg<DAC2_DATA_SPEC>`"]
pub type DAC2_DATA = crate::Reg<dac2_data::DAC2_DATA_SPEC>;
#[doc = "DAC2 Data Register"]
pub mod dac2_data;
