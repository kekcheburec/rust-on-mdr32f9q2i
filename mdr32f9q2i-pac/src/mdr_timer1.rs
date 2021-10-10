#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Counter Register"]
    pub cnt: crate::Reg<cnt::CNT_SPEC>,
    #[doc = "0x04 - Timer Clock Prescaler Register"]
    pub psg: crate::Reg<psg::PSG_SPEC>,
    #[doc = "0x08 - Timer Auto-Reload Register"]
    pub arr: crate::Reg<arr::ARR_SPEC>,
    #[doc = "0x0c - Timer Control Register"]
    pub cntrl: crate::Reg<cntrl::CNTRL_SPEC>,
    #[doc = "0x10 - Timer Capture/Compare Register"]
    pub ccr1: crate::Reg<ccr1::CCR1_SPEC>,
    #[doc = "0x14 - Timer Capture/Compare Register"]
    pub ccr2: crate::Reg<ccr2::CCR2_SPEC>,
    #[doc = "0x18 - Timer Capture/Compare Register"]
    pub ccr3: crate::Reg<ccr3::CCR3_SPEC>,
    #[doc = "0x1c - Timer Capture/Compare Register"]
    pub ccr4: crate::Reg<ccr4::CCR4_SPEC>,
    #[doc = "0x20 - Timer Channel Control Register"]
    pub ch1_cntrl: crate::Reg<ch1_cntrl::CH1_CNTRL_SPEC>,
    #[doc = "0x24 - Timer Channel Control Register"]
    pub ch2_cntrl: crate::Reg<ch2_cntrl::CH2_CNTRL_SPEC>,
    #[doc = "0x28 - Timer Channel Control Register"]
    pub ch3_cntrl: crate::Reg<ch3_cntrl::CH3_CNTRL_SPEC>,
    #[doc = "0x2c - Timer Channel Control Register"]
    pub ch4_cntrl: crate::Reg<ch4_cntrl::CH4_CNTRL_SPEC>,
    #[doc = "0x30 - Timer Channel Control1 Register"]
    pub ch1_cntrl1: crate::Reg<ch1_cntrl1::CH1_CNTRL1_SPEC>,
    #[doc = "0x34 - Timer Channel Control1 Register"]
    pub ch2_cntrl1: crate::Reg<ch2_cntrl1::CH2_CNTRL1_SPEC>,
    #[doc = "0x38 - Timer Channel Control1 Register"]
    pub ch3_cntrl1: crate::Reg<ch3_cntrl1::CH3_CNTRL1_SPEC>,
    #[doc = "0x3c - Timer Channel Control1 Register"]
    pub ch4_cntrl1: crate::Reg<ch4_cntrl1::CH4_CNTRL1_SPEC>,
    #[doc = "0x40 - Timer Channel DTG Register"]
    pub ch1_dtg: crate::Reg<ch1_dtg::CH1_DTG_SPEC>,
    #[doc = "0x44 - Timer Channel DTG Register"]
    pub ch2_dtg: crate::Reg<ch2_dtg::CH2_DTG_SPEC>,
    #[doc = "0x48 - Timer Channel DTG Register"]
    pub ch3_dtg: crate::Reg<ch3_dtg::CH3_DTG_SPEC>,
    #[doc = "0x4c - Timer Channel DTG Register"]
    pub ch4_dtg: crate::Reg<ch4_dtg::CH4_DTG_SPEC>,
    #[doc = "0x50 - Timer BRK/ETR Control Register"]
    pub brketr_cntrl: crate::Reg<brketr_cntrl::BRKETR_CNTRL_SPEC>,
    #[doc = "0x54 - Timer Status Register"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x58 - Timer Interrupt Enable Register"]
    pub ie: crate::Reg<ie::IE_SPEC>,
    #[doc = "0x5c - Timer DMA Request Enable Register"]
    pub dma_re: crate::Reg<dma_re::DMA_RE_SPEC>,
    #[doc = "0x60 - Timer Channel Control2 Register"]
    pub ch1_cntrl2: crate::Reg<ch1_cntrl2::CH1_CNTRL2_SPEC>,
    #[doc = "0x64 - Timer Channel Control2 Register"]
    pub ch2_cntrl2: crate::Reg<ch2_cntrl2::CH2_CNTRL2_SPEC>,
    #[doc = "0x68 - Timer Channel Control2 Register"]
    pub ch3_cntrl2: crate::Reg<ch3_cntrl2::CH3_CNTRL2_SPEC>,
    #[doc = "0x6c - Timer Channel Control2 Register"]
    pub ch4_cntrl2: crate::Reg<ch4_cntrl2::CH4_CNTRL2_SPEC>,
    #[doc = "0x70 - Timer Capture/Compare1 Register"]
    pub ccr11: crate::Reg<ccr11::CCR11_SPEC>,
    #[doc = "0x74 - Timer Capture/Compare1 Register"]
    pub ccr21: crate::Reg<ccr21::CCR21_SPEC>,
    #[doc = "0x78 - Timer Capture/Compare1 Register"]
    pub ccr31: crate::Reg<ccr31::CCR31_SPEC>,
    #[doc = "0x7c - Timer Capture/Compare1 Register"]
    pub ccr41: crate::Reg<ccr41::CCR41_SPEC>,
}
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "Timer Counter Register"]
pub mod cnt;
#[doc = "PSG register accessor: an alias for `Reg<PSG_SPEC>`"]
pub type PSG = crate::Reg<psg::PSG_SPEC>;
#[doc = "Timer Clock Prescaler Register"]
pub mod psg;
#[doc = "ARR register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "Timer Auto-Reload Register"]
pub mod arr;
#[doc = "CNTRL register accessor: an alias for `Reg<CNTRL_SPEC>`"]
pub type CNTRL = crate::Reg<cntrl::CNTRL_SPEC>;
#[doc = "Timer Control Register"]
pub mod cntrl;
#[doc = "CCR1 register accessor: an alias for `Reg<CCR1_SPEC>`"]
pub type CCR1 = crate::Reg<ccr1::CCR1_SPEC>;
#[doc = "Timer Capture/Compare Register"]
pub mod ccr1;
#[doc = "CCR2 register accessor: an alias for `Reg<CCR2_SPEC>`"]
pub type CCR2 = crate::Reg<ccr2::CCR2_SPEC>;
#[doc = "Timer Capture/Compare Register"]
pub mod ccr2;
#[doc = "CCR3 register accessor: an alias for `Reg<CCR3_SPEC>`"]
pub type CCR3 = crate::Reg<ccr3::CCR3_SPEC>;
#[doc = "Timer Capture/Compare Register"]
pub mod ccr3;
#[doc = "CCR4 register accessor: an alias for `Reg<CCR4_SPEC>`"]
pub type CCR4 = crate::Reg<ccr4::CCR4_SPEC>;
#[doc = "Timer Capture/Compare Register"]
pub mod ccr4;
#[doc = "CH1_CNTRL register accessor: an alias for `Reg<CH1_CNTRL_SPEC>`"]
pub type CH1_CNTRL = crate::Reg<ch1_cntrl::CH1_CNTRL_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ch1_cntrl;
#[doc = "CH2_CNTRL register accessor: an alias for `Reg<CH2_CNTRL_SPEC>`"]
pub type CH2_CNTRL = crate::Reg<ch2_cntrl::CH2_CNTRL_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ch2_cntrl;
#[doc = "CH3_CNTRL register accessor: an alias for `Reg<CH3_CNTRL_SPEC>`"]
pub type CH3_CNTRL = crate::Reg<ch3_cntrl::CH3_CNTRL_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ch3_cntrl;
#[doc = "CH4_CNTRL register accessor: an alias for `Reg<CH4_CNTRL_SPEC>`"]
pub type CH4_CNTRL = crate::Reg<ch4_cntrl::CH4_CNTRL_SPEC>;
#[doc = "Timer Channel Control Register"]
pub mod ch4_cntrl;
#[doc = "CH1_CNTRL1 register accessor: an alias for `Reg<CH1_CNTRL1_SPEC>`"]
pub type CH1_CNTRL1 = crate::Reg<ch1_cntrl1::CH1_CNTRL1_SPEC>;
#[doc = "Timer Channel Control1 Register"]
pub mod ch1_cntrl1;
#[doc = "CH2_CNTRL1 register accessor: an alias for `Reg<CH2_CNTRL1_SPEC>`"]
pub type CH2_CNTRL1 = crate::Reg<ch2_cntrl1::CH2_CNTRL1_SPEC>;
#[doc = "Timer Channel Control1 Register"]
pub mod ch2_cntrl1;
#[doc = "CH3_CNTRL1 register accessor: an alias for `Reg<CH3_CNTRL1_SPEC>`"]
pub type CH3_CNTRL1 = crate::Reg<ch3_cntrl1::CH3_CNTRL1_SPEC>;
#[doc = "Timer Channel Control1 Register"]
pub mod ch3_cntrl1;
#[doc = "CH4_CNTRL1 register accessor: an alias for `Reg<CH4_CNTRL1_SPEC>`"]
pub type CH4_CNTRL1 = crate::Reg<ch4_cntrl1::CH4_CNTRL1_SPEC>;
#[doc = "Timer Channel Control1 Register"]
pub mod ch4_cntrl1;
#[doc = "CH1_DTG register accessor: an alias for `Reg<CH1_DTG_SPEC>`"]
pub type CH1_DTG = crate::Reg<ch1_dtg::CH1_DTG_SPEC>;
#[doc = "Timer Channel DTG Register"]
pub mod ch1_dtg;
#[doc = "CH2_DTG register accessor: an alias for `Reg<CH2_DTG_SPEC>`"]
pub type CH2_DTG = crate::Reg<ch2_dtg::CH2_DTG_SPEC>;
#[doc = "Timer Channel DTG Register"]
pub mod ch2_dtg;
#[doc = "CH3_DTG register accessor: an alias for `Reg<CH3_DTG_SPEC>`"]
pub type CH3_DTG = crate::Reg<ch3_dtg::CH3_DTG_SPEC>;
#[doc = "Timer Channel DTG Register"]
pub mod ch3_dtg;
#[doc = "CH4_DTG register accessor: an alias for `Reg<CH4_DTG_SPEC>`"]
pub type CH4_DTG = crate::Reg<ch4_dtg::CH4_DTG_SPEC>;
#[doc = "Timer Channel DTG Register"]
pub mod ch4_dtg;
#[doc = "BRKETR_CNTRL register accessor: an alias for `Reg<BRKETR_CNTRL_SPEC>`"]
pub type BRKETR_CNTRL = crate::Reg<brketr_cntrl::BRKETR_CNTRL_SPEC>;
#[doc = "Timer BRK/ETR Control Register"]
pub mod brketr_cntrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Timer Status Register"]
pub mod status;
#[doc = "IE register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Timer Interrupt Enable Register"]
pub mod ie;
#[doc = "DMA_RE register accessor: an alias for `Reg<DMA_RE_SPEC>`"]
pub type DMA_RE = crate::Reg<dma_re::DMA_RE_SPEC>;
#[doc = "Timer DMA Request Enable Register"]
pub mod dma_re;
#[doc = "CH1_CNTRL2 register accessor: an alias for `Reg<CH1_CNTRL2_SPEC>`"]
pub type CH1_CNTRL2 = crate::Reg<ch1_cntrl2::CH1_CNTRL2_SPEC>;
#[doc = "Timer Channel Control2 Register"]
pub mod ch1_cntrl2;
#[doc = "CH2_CNTRL2 register accessor: an alias for `Reg<CH2_CNTRL2_SPEC>`"]
pub type CH2_CNTRL2 = crate::Reg<ch2_cntrl2::CH2_CNTRL2_SPEC>;
#[doc = "Timer Channel Control2 Register"]
pub mod ch2_cntrl2;
#[doc = "CH3_CNTRL2 register accessor: an alias for `Reg<CH3_CNTRL2_SPEC>`"]
pub type CH3_CNTRL2 = crate::Reg<ch3_cntrl2::CH3_CNTRL2_SPEC>;
#[doc = "Timer Channel Control2 Register"]
pub mod ch3_cntrl2;
#[doc = "CH4_CNTRL2 register accessor: an alias for `Reg<CH4_CNTRL2_SPEC>`"]
pub type CH4_CNTRL2 = crate::Reg<ch4_cntrl2::CH4_CNTRL2_SPEC>;
#[doc = "Timer Channel Control2 Register"]
pub mod ch4_cntrl2;
#[doc = "CCR11 register accessor: an alias for `Reg<CCR11_SPEC>`"]
pub type CCR11 = crate::Reg<ccr11::CCR11_SPEC>;
#[doc = "Timer Capture/Compare1 Register"]
pub mod ccr11;
#[doc = "CCR21 register accessor: an alias for `Reg<CCR21_SPEC>`"]
pub type CCR21 = crate::Reg<ccr21::CCR21_SPEC>;
#[doc = "Timer Capture/Compare1 Register"]
pub mod ccr21;
#[doc = "CCR31 register accessor: an alias for `Reg<CCR31_SPEC>`"]
pub type CCR31 = crate::Reg<ccr31::CCR31_SPEC>;
#[doc = "Timer Capture/Compare1 Register"]
pub mod ccr31;
#[doc = "CCR41 register accessor: an alias for `Reg<CCR41_SPEC>`"]
pub type CCR41 = crate::Reg<ccr41::CCR41_SPEC>;
#[doc = "Timer Capture/Compare1 Register"]
pub mod ccr41;
