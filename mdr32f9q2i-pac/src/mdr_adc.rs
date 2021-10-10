#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC1 Configuration Register"]
    pub adc1_cfg: crate::Reg<adc1_cfg::ADC1_CFG_SPEC>,
    #[doc = "0x04 - ADC2 Configuration Register"]
    pub adc2_cfg: crate::Reg<adc2_cfg::ADC2_CFG_SPEC>,
    #[doc = "0x08 - ADC1 High Level Register"]
    pub adc1_h_level: crate::Reg<adc1_h_level::ADC1_H_LEVEL_SPEC>,
    #[doc = "0x0c - ADC2 High Level Register"]
    pub adc2_h_level: crate::Reg<adc2_h_level::ADC2_H_LEVEL_SPEC>,
    #[doc = "0x10 - ADC1 Low Level Register"]
    pub adc1_l_level: crate::Reg<adc1_l_level::ADC1_L_LEVEL_SPEC>,
    #[doc = "0x14 - ADC2 Low Level Register"]
    pub adc2_l_level: crate::Reg<adc2_l_level::ADC2_L_LEVEL_SPEC>,
    #[doc = "0x18 - ADC1 Result Register"]
    pub adc1_result: crate::Reg<adc1_result::ADC1_RESULT_SPEC>,
    #[doc = "0x1c - ADC2 Result Register"]
    pub adc2_result: crate::Reg<adc2_result::ADC2_RESULT_SPEC>,
    #[doc = "0x20 - ADC1 Status Register"]
    pub adc1_status: crate::Reg<adc1_status::ADC1_STATUS_SPEC>,
    #[doc = "0x24 - ADC2 Status Register"]
    pub adc2_status: crate::Reg<adc2_status::ADC2_STATUS_SPEC>,
    #[doc = "0x28 - ADC1 Channel Selector Register"]
    pub adc1_chsel: crate::Reg<adc1_chsel::ADC1_CHSEL_SPEC>,
    #[doc = "0x2c - ADC2 Channel Selector Register"]
    pub adc2_chsel: crate::Reg<adc2_chsel::ADC2_CHSEL_SPEC>,
}
#[doc = "ADC1_CFG register accessor: an alias for `Reg<ADC1_CFG_SPEC>`"]
pub type ADC1_CFG = crate::Reg<adc1_cfg::ADC1_CFG_SPEC>;
#[doc = "ADC1 Configuration Register"]
pub mod adc1_cfg;
#[doc = "ADC2_CFG register accessor: an alias for `Reg<ADC2_CFG_SPEC>`"]
pub type ADC2_CFG = crate::Reg<adc2_cfg::ADC2_CFG_SPEC>;
#[doc = "ADC2 Configuration Register"]
pub mod adc2_cfg;
#[doc = "ADC1_H_LEVEL register accessor: an alias for `Reg<ADC1_H_LEVEL_SPEC>`"]
pub type ADC1_H_LEVEL = crate::Reg<adc1_h_level::ADC1_H_LEVEL_SPEC>;
#[doc = "ADC1 High Level Register"]
pub mod adc1_h_level;
#[doc = "ADC2_H_LEVEL register accessor: an alias for `Reg<ADC2_H_LEVEL_SPEC>`"]
pub type ADC2_H_LEVEL = crate::Reg<adc2_h_level::ADC2_H_LEVEL_SPEC>;
#[doc = "ADC2 High Level Register"]
pub mod adc2_h_level;
#[doc = "ADC1_L_LEVEL register accessor: an alias for `Reg<ADC1_L_LEVEL_SPEC>`"]
pub type ADC1_L_LEVEL = crate::Reg<adc1_l_level::ADC1_L_LEVEL_SPEC>;
#[doc = "ADC1 Low Level Register"]
pub mod adc1_l_level;
#[doc = "ADC2_L_LEVEL register accessor: an alias for `Reg<ADC2_L_LEVEL_SPEC>`"]
pub type ADC2_L_LEVEL = crate::Reg<adc2_l_level::ADC2_L_LEVEL_SPEC>;
#[doc = "ADC2 Low Level Register"]
pub mod adc2_l_level;
#[doc = "ADC1_RESULT register accessor: an alias for `Reg<ADC1_RESULT_SPEC>`"]
pub type ADC1_RESULT = crate::Reg<adc1_result::ADC1_RESULT_SPEC>;
#[doc = "ADC1 Result Register"]
pub mod adc1_result;
#[doc = "ADC2_RESULT register accessor: an alias for `Reg<ADC2_RESULT_SPEC>`"]
pub type ADC2_RESULT = crate::Reg<adc2_result::ADC2_RESULT_SPEC>;
#[doc = "ADC2 Result Register"]
pub mod adc2_result;
#[doc = "ADC1_STATUS register accessor: an alias for `Reg<ADC1_STATUS_SPEC>`"]
pub type ADC1_STATUS = crate::Reg<adc1_status::ADC1_STATUS_SPEC>;
#[doc = "ADC1 Status Register"]
pub mod adc1_status;
#[doc = "ADC2_STATUS register accessor: an alias for `Reg<ADC2_STATUS_SPEC>`"]
pub type ADC2_STATUS = crate::Reg<adc2_status::ADC2_STATUS_SPEC>;
#[doc = "ADC2 Status Register"]
pub mod adc2_status;
#[doc = "ADC1_CHSEL register accessor: an alias for `Reg<ADC1_CHSEL_SPEC>`"]
pub type ADC1_CHSEL = crate::Reg<adc1_chsel::ADC1_CHSEL_SPEC>;
#[doc = "ADC1 Channel Selector Register"]
pub mod adc1_chsel;
#[doc = "ADC2_CHSEL register accessor: an alias for `Reg<ADC2_CHSEL_SPEC>`"]
pub type ADC2_CHSEL = crate::Reg<adc2_chsel::ADC2_CHSEL_SPEC>;
#[doc = "ADC2 Channel Selector Register"]
pub mod adc2_chsel;
