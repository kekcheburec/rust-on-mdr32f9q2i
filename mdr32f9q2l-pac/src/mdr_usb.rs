#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB HTXC Register"]
    pub htxc: crate::Reg<htxc::HTXC_SPEC>,
    #[doc = "0x04 - USB HTXT Register"]
    pub htxt: crate::Reg<htxt::HTXT_SPEC>,
    #[doc = "0x08 - USB HTXLC Register"]
    pub htxlc: crate::Reg<htxlc::HTXLC_SPEC>,
    #[doc = "0x0c - "]
    pub htxse: crate::Reg<htxse::HTXSE_SPEC>,
    #[doc = "0x10 - "]
    pub htxa: crate::Reg<htxa::HTXA_SPEC>,
    #[doc = "0x14 - "]
    pub htxe: crate::Reg<htxe::HTXE_SPEC>,
    #[doc = "0x18 - "]
    pub hfn_l: crate::Reg<hfn_l::HFN_L_SPEC>,
    #[doc = "0x1c - "]
    pub hfn_h: crate::Reg<hfn_h::HFN_H_SPEC>,
    #[doc = "0x20 - USB_HIS Register"]
    pub his: crate::Reg<his::HIS_SPEC>,
    #[doc = "0x24 - USB_HIM Register"]
    pub him: crate::Reg<him::HIM_SPEC>,
    #[doc = "0x28 - USB_HRXS Register"]
    pub hrxs: crate::Reg<hrxs::HRXS_SPEC>,
    #[doc = "0x2c - "]
    pub hrxp: crate::Reg<hrxp::HRXP_SPEC>,
    #[doc = "0x30 - "]
    pub hrxa: crate::Reg<hrxa::HRXA_SPEC>,
    #[doc = "0x34 - "]
    pub hrxe: crate::Reg<hrxe::HRXE_SPEC>,
    #[doc = "0x38 - "]
    pub hrxcs: crate::Reg<hrxcs::HRXCS_SPEC>,
    #[doc = "0x3c - "]
    pub hstm: crate::Reg<hstm::HSTM_SPEC>,
    _reserved16: [u8; 0x40],
    #[doc = "0x80 - "]
    pub hrxfd: crate::Reg<hrxfd::HRXFD_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x88 - "]
    pub hrxfdc_l: crate::Reg<hrxfdc_l::HRXFDC_L_SPEC>,
    #[doc = "0x8c - "]
    pub hrxfdc_h: crate::Reg<hrxfdc_h::HRXFDC_H_SPEC>,
    #[doc = "0x90 - "]
    pub hrxfc: crate::Reg<hrxfc::HRXFC_SPEC>,
    _reserved20: [u8; 0x2c],
    #[doc = "0xc0 - "]
    pub htxfd: crate::Reg<htxfd::HTXFD_SPEC>,
    _reserved21: [u8; 0x0c],
    #[doc = "0xd0 - "]
    pub htxfc: crate::Reg<htxfc::HTXFC_SPEC>,
    _reserved22: [u8; 0x2c],
    #[doc = "0x100 - USB_SEP Control Register"]
    pub sep0_ctrl: crate::Reg<sep0_ctrl::SEP0_CTRL_SPEC>,
    #[doc = "0x104 - USB_SEP Status Register"]
    pub sep0_sts: crate::Reg<sep0_sts::SEP0_STS_SPEC>,
    #[doc = "0x108 - "]
    pub sep0_ts: crate::Reg<sep0_ts::SEP0_TS_SPEC>,
    #[doc = "0x10c - "]
    pub sep0_nts: crate::Reg<sep0_nts::SEP0_NTS_SPEC>,
    #[doc = "0x110 - USB_SEP Control Register"]
    pub sep1_ctrl: crate::Reg<sep1_ctrl::SEP1_CTRL_SPEC>,
    #[doc = "0x114 - USB_SEP Status Register"]
    pub sep1_sts: crate::Reg<sep1_sts::SEP1_STS_SPEC>,
    #[doc = "0x118 - "]
    pub sep1_ts: crate::Reg<sep1_ts::SEP1_TS_SPEC>,
    #[doc = "0x11c - "]
    pub sep1_nts: crate::Reg<sep1_nts::SEP1_NTS_SPEC>,
    #[doc = "0x120 - USB_SEP Control Register"]
    pub sep2_ctrl: crate::Reg<sep2_ctrl::SEP2_CTRL_SPEC>,
    #[doc = "0x124 - USB_SEP Status Register"]
    pub sep2_sts: crate::Reg<sep2_sts::SEP2_STS_SPEC>,
    #[doc = "0x128 - "]
    pub sep2_ts: crate::Reg<sep2_ts::SEP2_TS_SPEC>,
    #[doc = "0x12c - "]
    pub sep2_nts: crate::Reg<sep2_nts::SEP2_NTS_SPEC>,
    #[doc = "0x130 - USB_SEP Control Register"]
    pub sep3_ctrl: crate::Reg<sep3_ctrl::SEP3_CTRL_SPEC>,
    #[doc = "0x134 - USB_SEP Status Register"]
    pub sep3_sts: crate::Reg<sep3_sts::SEP3_STS_SPEC>,
    #[doc = "0x138 - "]
    pub sep3_ts: crate::Reg<sep3_ts::SEP3_TS_SPEC>,
    #[doc = "0x13c - "]
    pub sep3_nts: crate::Reg<sep3_nts::SEP3_NTS_SPEC>,
    #[doc = "0x140 - USB_SC Register"]
    pub sc: crate::Reg<sc::SC_SPEC>,
    #[doc = "0x144 - "]
    pub sls: crate::Reg<sls::SLS_SPEC>,
    #[doc = "0x148 - USB_SIS Register"]
    pub sis: crate::Reg<sis::SIS_SPEC>,
    #[doc = "0x14c - USB_SIM Register"]
    pub sim: crate::Reg<sim::SIM_SPEC>,
    #[doc = "0x150 - "]
    pub sa: crate::Reg<sa::SA_SPEC>,
    #[doc = "0x154 - "]
    pub sfn_l: crate::Reg<sfn_l::SFN_L_SPEC>,
    #[doc = "0x158 - "]
    pub sfn_h: crate::Reg<sfn_h::SFN_H_SPEC>,
    _reserved45: [u8; 0x24],
    #[doc = "0x180 - "]
    pub sep_fifo0_rxfd: crate::Reg<sep_fifo0_rxfd::SEP_FIFO0_RXFD_SPEC>,
    _reserved46: [u8; 0x04],
    #[doc = "0x188 - "]
    pub sep_fifo0_rxfdc_l: crate::Reg<sep_fifo0_rxfdc_l::SEP_FIFO0_RXFDC_L_SPEC>,
    #[doc = "0x18c - "]
    pub sep_fifo0_rxfdc_h: crate::Reg<sep_fifo0_rxfdc_h::SEP_FIFO0_RXFDC_H_SPEC>,
    #[doc = "0x190 - "]
    pub sep_fifo0_rxfc: crate::Reg<sep_fifo0_rxfc::SEP_FIFO0_RXFC_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x198 - "]
    pub sep_fifo1_rxfd: crate::Reg<sep_fifo1_rxfd::SEP_FIFO1_RXFD_SPEC>,
    _reserved50: [u8; 0x04],
    #[doc = "0x1a0 - "]
    pub sep_fifo1_rxfdc_l: crate::Reg<sep_fifo1_rxfdc_l::SEP_FIFO1_RXFDC_L_SPEC>,
    #[doc = "0x1a4 - "]
    pub sep_fifo1_rxfdc_h: crate::Reg<sep_fifo1_rxfdc_h::SEP_FIFO1_RXFDC_H_SPEC>,
    #[doc = "0x1a8 - "]
    pub sep_fifo1_rxfc: crate::Reg<sep_fifo1_rxfc::SEP_FIFO1_RXFC_SPEC>,
    _reserved53: [u8; 0x04],
    #[doc = "0x1b0 - "]
    pub sep_fifo2_rxfd: crate::Reg<sep_fifo2_rxfd::SEP_FIFO2_RXFD_SPEC>,
    _reserved54: [u8; 0x04],
    #[doc = "0x1b8 - "]
    pub sep_fifo2_rxfdc_l: crate::Reg<sep_fifo2_rxfdc_l::SEP_FIFO2_RXFDC_L_SPEC>,
    #[doc = "0x1bc - "]
    pub sep_fifo2_rxfdc_h: crate::Reg<sep_fifo2_rxfdc_h::SEP_FIFO2_RXFDC_H_SPEC>,
    #[doc = "0x1c0 - "]
    pub sep_fifo0_txfd: crate::Reg<sep_fifo0_txfd::SEP_FIFO0_TXFD_SPEC>,
    _reserved57: [u8; 0x04],
    #[doc = "0x1c8 - "]
    pub sep_fifo3_rxfd: crate::Reg<sep_fifo3_rxfd::SEP_FIFO3_RXFD_SPEC>,
    _reserved58: [u8; 0x04],
    #[doc = "0x1d0 - "]
    pub sep_fifo0_txfdc: crate::Reg<sep_fifo0_txfdc::SEP_FIFO0_TXFDC_SPEC>,
    #[doc = "0x1d4 - "]
    pub sep_fifo3_rxfdc_h: crate::Reg<sep_fifo3_rxfdc_h::SEP_FIFO3_RXFDC_H_SPEC>,
    #[doc = "0x1d8 - "]
    pub sep_fifo1_txfd: crate::Reg<sep_fifo1_txfd::SEP_FIFO1_TXFD_SPEC>,
    _reserved61: [u8; 0x0c],
    #[doc = "0x1e8 - "]
    pub sep_fifo1_txfdc: crate::Reg<sep_fifo1_txfdc::SEP_FIFO1_TXFDC_SPEC>,
    _reserved62: [u8; 0x04],
    #[doc = "0x1f0 - "]
    pub sep_fifo2_txfd: crate::Reg<sep_fifo2_txfd::SEP_FIFO2_TXFD_SPEC>,
    _reserved63: [u8; 0x0c],
    #[doc = "0x200 - "]
    pub sep_fifo2_txfdc: crate::Reg<sep_fifo2_txfdc::SEP_FIFO2_TXFDC_SPEC>,
    _reserved64: [u8; 0x04],
    #[doc = "0x208 - "]
    pub sep_fifo3_txfd: crate::Reg<sep_fifo3_txfd::SEP_FIFO3_TXFD_SPEC>,
    _reserved65: [u8; 0x0c],
    #[doc = "0x218 - "]
    pub sep_fifo3_txfdc: crate::Reg<sep_fifo3_txfdc::SEP_FIFO3_TXFDC_SPEC>,
    _reserved66: [u8; 0x74],
    #[doc = "0x290 - "]
    pub sep_fifo2_rxfc: crate::Reg<sep_fifo2_rxfc::SEP_FIFO2_RXFC_SPEC>,
    _reserved67: [u8; 0x74],
    #[doc = "0x308 - "]
    pub sep_fifo3_rxfdc_l: crate::Reg<sep_fifo3_rxfdc_l::SEP_FIFO3_RXFDC_L_SPEC>,
    _reserved68: [u8; 0x04],
    #[doc = "0x310 - "]
    pub sep_fifo3_rxfc: crate::Reg<sep_fifo3_rxfc::SEP_FIFO3_RXFC_SPEC>,
    _reserved69: [u8; 0x6c],
    #[doc = "0x380 - USB_HSCR Register"]
    pub hscr: crate::Reg<hscr::HSCR_SPEC>,
    #[doc = "0x384 - USB_HSVR Register"]
    pub hsvr: crate::Reg<hsvr::HSVR_SPEC>,
}
#[doc = "HTXC register accessor: an alias for `Reg<HTXC_SPEC>`"]
pub type HTXC = crate::Reg<htxc::HTXC_SPEC>;
#[doc = "USB HTXC Register"]
pub mod htxc;
#[doc = "HTXT register accessor: an alias for `Reg<HTXT_SPEC>`"]
pub type HTXT = crate::Reg<htxt::HTXT_SPEC>;
#[doc = "USB HTXT Register"]
pub mod htxt;
#[doc = "HTXLC register accessor: an alias for `Reg<HTXLC_SPEC>`"]
pub type HTXLC = crate::Reg<htxlc::HTXLC_SPEC>;
#[doc = "USB HTXLC Register"]
pub mod htxlc;
#[doc = "HTXSE register accessor: an alias for `Reg<HTXSE_SPEC>`"]
pub type HTXSE = crate::Reg<htxse::HTXSE_SPEC>;
#[doc = ""]
pub mod htxse;
#[doc = "HTXA register accessor: an alias for `Reg<HTXA_SPEC>`"]
pub type HTXA = crate::Reg<htxa::HTXA_SPEC>;
#[doc = ""]
pub mod htxa;
#[doc = "HTXE register accessor: an alias for `Reg<HTXE_SPEC>`"]
pub type HTXE = crate::Reg<htxe::HTXE_SPEC>;
#[doc = ""]
pub mod htxe;
#[doc = "HFN_L register accessor: an alias for `Reg<HFN_L_SPEC>`"]
pub type HFN_L = crate::Reg<hfn_l::HFN_L_SPEC>;
#[doc = ""]
pub mod hfn_l;
#[doc = "HFN_H register accessor: an alias for `Reg<HFN_H_SPEC>`"]
pub type HFN_H = crate::Reg<hfn_h::HFN_H_SPEC>;
#[doc = ""]
pub mod hfn_h;
#[doc = "HIS register accessor: an alias for `Reg<HIS_SPEC>`"]
pub type HIS = crate::Reg<his::HIS_SPEC>;
#[doc = "USB_HIS Register"]
pub mod his;
#[doc = "HIM register accessor: an alias for `Reg<HIM_SPEC>`"]
pub type HIM = crate::Reg<him::HIM_SPEC>;
#[doc = "USB_HIM Register"]
pub mod him;
#[doc = "HRXS register accessor: an alias for `Reg<HRXS_SPEC>`"]
pub type HRXS = crate::Reg<hrxs::HRXS_SPEC>;
#[doc = "USB_HRXS Register"]
pub mod hrxs;
#[doc = "HRXP register accessor: an alias for `Reg<HRXP_SPEC>`"]
pub type HRXP = crate::Reg<hrxp::HRXP_SPEC>;
#[doc = ""]
pub mod hrxp;
#[doc = "HRXA register accessor: an alias for `Reg<HRXA_SPEC>`"]
pub type HRXA = crate::Reg<hrxa::HRXA_SPEC>;
#[doc = ""]
pub mod hrxa;
#[doc = "HRXE register accessor: an alias for `Reg<HRXE_SPEC>`"]
pub type HRXE = crate::Reg<hrxe::HRXE_SPEC>;
#[doc = ""]
pub mod hrxe;
#[doc = "HRXCS register accessor: an alias for `Reg<HRXCS_SPEC>`"]
pub type HRXCS = crate::Reg<hrxcs::HRXCS_SPEC>;
#[doc = ""]
pub mod hrxcs;
#[doc = "HSTM register accessor: an alias for `Reg<HSTM_SPEC>`"]
pub type HSTM = crate::Reg<hstm::HSTM_SPEC>;
#[doc = ""]
pub mod hstm;
#[doc = "HRXFD register accessor: an alias for `Reg<HRXFD_SPEC>`"]
pub type HRXFD = crate::Reg<hrxfd::HRXFD_SPEC>;
#[doc = ""]
pub mod hrxfd;
#[doc = "HRXFDC_L register accessor: an alias for `Reg<HRXFDC_L_SPEC>`"]
pub type HRXFDC_L = crate::Reg<hrxfdc_l::HRXFDC_L_SPEC>;
#[doc = ""]
pub mod hrxfdc_l;
#[doc = "HRXFDC_H register accessor: an alias for `Reg<HRXFDC_H_SPEC>`"]
pub type HRXFDC_H = crate::Reg<hrxfdc_h::HRXFDC_H_SPEC>;
#[doc = ""]
pub mod hrxfdc_h;
#[doc = "HRXFC register accessor: an alias for `Reg<HRXFC_SPEC>`"]
pub type HRXFC = crate::Reg<hrxfc::HRXFC_SPEC>;
#[doc = ""]
pub mod hrxfc;
#[doc = "HTXFD register accessor: an alias for `Reg<HTXFD_SPEC>`"]
pub type HTXFD = crate::Reg<htxfd::HTXFD_SPEC>;
#[doc = ""]
pub mod htxfd;
#[doc = "HTXFC register accessor: an alias for `Reg<HTXFC_SPEC>`"]
pub type HTXFC = crate::Reg<htxfc::HTXFC_SPEC>;
#[doc = ""]
pub mod htxfc;
#[doc = "SEP0_CTRL register accessor: an alias for `Reg<SEP0_CTRL_SPEC>`"]
pub type SEP0_CTRL = crate::Reg<sep0_ctrl::SEP0_CTRL_SPEC>;
#[doc = "USB_SEP Control Register"]
pub mod sep0_ctrl;
#[doc = "SEP0_STS register accessor: an alias for `Reg<SEP0_STS_SPEC>`"]
pub type SEP0_STS = crate::Reg<sep0_sts::SEP0_STS_SPEC>;
#[doc = "USB_SEP Status Register"]
pub mod sep0_sts;
#[doc = "SEP0_TS register accessor: an alias for `Reg<SEP0_TS_SPEC>`"]
pub type SEP0_TS = crate::Reg<sep0_ts::SEP0_TS_SPEC>;
#[doc = ""]
pub mod sep0_ts;
#[doc = "SEP0_NTS register accessor: an alias for `Reg<SEP0_NTS_SPEC>`"]
pub type SEP0_NTS = crate::Reg<sep0_nts::SEP0_NTS_SPEC>;
#[doc = ""]
pub mod sep0_nts;
#[doc = "SEP1_CTRL register accessor: an alias for `Reg<SEP1_CTRL_SPEC>`"]
pub type SEP1_CTRL = crate::Reg<sep1_ctrl::SEP1_CTRL_SPEC>;
#[doc = "USB_SEP Control Register"]
pub mod sep1_ctrl;
#[doc = "SEP1_STS register accessor: an alias for `Reg<SEP1_STS_SPEC>`"]
pub type SEP1_STS = crate::Reg<sep1_sts::SEP1_STS_SPEC>;
#[doc = "USB_SEP Status Register"]
pub mod sep1_sts;
#[doc = "SEP1_TS register accessor: an alias for `Reg<SEP1_TS_SPEC>`"]
pub type SEP1_TS = crate::Reg<sep1_ts::SEP1_TS_SPEC>;
#[doc = ""]
pub mod sep1_ts;
#[doc = "SEP1_NTS register accessor: an alias for `Reg<SEP1_NTS_SPEC>`"]
pub type SEP1_NTS = crate::Reg<sep1_nts::SEP1_NTS_SPEC>;
#[doc = ""]
pub mod sep1_nts;
#[doc = "SEP2_CTRL register accessor: an alias for `Reg<SEP2_CTRL_SPEC>`"]
pub type SEP2_CTRL = crate::Reg<sep2_ctrl::SEP2_CTRL_SPEC>;
#[doc = "USB_SEP Control Register"]
pub mod sep2_ctrl;
#[doc = "SEP2_STS register accessor: an alias for `Reg<SEP2_STS_SPEC>`"]
pub type SEP2_STS = crate::Reg<sep2_sts::SEP2_STS_SPEC>;
#[doc = "USB_SEP Status Register"]
pub mod sep2_sts;
#[doc = "SEP2_TS register accessor: an alias for `Reg<SEP2_TS_SPEC>`"]
pub type SEP2_TS = crate::Reg<sep2_ts::SEP2_TS_SPEC>;
#[doc = ""]
pub mod sep2_ts;
#[doc = "SEP2_NTS register accessor: an alias for `Reg<SEP2_NTS_SPEC>`"]
pub type SEP2_NTS = crate::Reg<sep2_nts::SEP2_NTS_SPEC>;
#[doc = ""]
pub mod sep2_nts;
#[doc = "SEP3_CTRL register accessor: an alias for `Reg<SEP3_CTRL_SPEC>`"]
pub type SEP3_CTRL = crate::Reg<sep3_ctrl::SEP3_CTRL_SPEC>;
#[doc = "USB_SEP Control Register"]
pub mod sep3_ctrl;
#[doc = "SEP3_STS register accessor: an alias for `Reg<SEP3_STS_SPEC>`"]
pub type SEP3_STS = crate::Reg<sep3_sts::SEP3_STS_SPEC>;
#[doc = "USB_SEP Status Register"]
pub mod sep3_sts;
#[doc = "SEP3_TS register accessor: an alias for `Reg<SEP3_TS_SPEC>`"]
pub type SEP3_TS = crate::Reg<sep3_ts::SEP3_TS_SPEC>;
#[doc = ""]
pub mod sep3_ts;
#[doc = "SEP3_NTS register accessor: an alias for `Reg<SEP3_NTS_SPEC>`"]
pub type SEP3_NTS = crate::Reg<sep3_nts::SEP3_NTS_SPEC>;
#[doc = ""]
pub mod sep3_nts;
#[doc = "SC register accessor: an alias for `Reg<SC_SPEC>`"]
pub type SC = crate::Reg<sc::SC_SPEC>;
#[doc = "USB_SC Register"]
pub mod sc;
#[doc = "SLS register accessor: an alias for `Reg<SLS_SPEC>`"]
pub type SLS = crate::Reg<sls::SLS_SPEC>;
#[doc = ""]
pub mod sls;
#[doc = "SIS register accessor: an alias for `Reg<SIS_SPEC>`"]
pub type SIS = crate::Reg<sis::SIS_SPEC>;
#[doc = "USB_SIS Register"]
pub mod sis;
#[doc = "SIM register accessor: an alias for `Reg<SIM_SPEC>`"]
pub type SIM = crate::Reg<sim::SIM_SPEC>;
#[doc = "USB_SIM Register"]
pub mod sim;
#[doc = "SA register accessor: an alias for `Reg<SA_SPEC>`"]
pub type SA = crate::Reg<sa::SA_SPEC>;
#[doc = ""]
pub mod sa;
#[doc = "SFN_L register accessor: an alias for `Reg<SFN_L_SPEC>`"]
pub type SFN_L = crate::Reg<sfn_l::SFN_L_SPEC>;
#[doc = ""]
pub mod sfn_l;
#[doc = "SFN_H register accessor: an alias for `Reg<SFN_H_SPEC>`"]
pub type SFN_H = crate::Reg<sfn_h::SFN_H_SPEC>;
#[doc = ""]
pub mod sfn_h;
#[doc = "SEP_FIFO0_RXFD register accessor: an alias for `Reg<SEP_FIFO0_RXFD_SPEC>`"]
pub type SEP_FIFO0_RXFD = crate::Reg<sep_fifo0_rxfd::SEP_FIFO0_RXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfd;
#[doc = "SEP_FIFO0_RXFDC_L register accessor: an alias for `Reg<SEP_FIFO0_RXFDC_L_SPEC>`"]
pub type SEP_FIFO0_RXFDC_L = crate::Reg<sep_fifo0_rxfdc_l::SEP_FIFO0_RXFDC_L_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfdc_l;
#[doc = "SEP_FIFO0_RXFDC_H register accessor: an alias for `Reg<SEP_FIFO0_RXFDC_H_SPEC>`"]
pub type SEP_FIFO0_RXFDC_H = crate::Reg<sep_fifo0_rxfdc_h::SEP_FIFO0_RXFDC_H_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfdc_h;
#[doc = "SEP_FIFO0_RXFC register accessor: an alias for `Reg<SEP_FIFO0_RXFC_SPEC>`"]
pub type SEP_FIFO0_RXFC = crate::Reg<sep_fifo0_rxfc::SEP_FIFO0_RXFC_SPEC>;
#[doc = ""]
pub mod sep_fifo0_rxfc;
#[doc = "SEP_FIFO0_TXFD register accessor: an alias for `Reg<SEP_FIFO0_TXFD_SPEC>`"]
pub type SEP_FIFO0_TXFD = crate::Reg<sep_fifo0_txfd::SEP_FIFO0_TXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo0_txfd;
#[doc = "SEP_FIFO0_TXFDC register accessor: an alias for `Reg<SEP_FIFO0_TXFDC_SPEC>`"]
pub type SEP_FIFO0_TXFDC = crate::Reg<sep_fifo0_txfdc::SEP_FIFO0_TXFDC_SPEC>;
#[doc = ""]
pub mod sep_fifo0_txfdc;
#[doc = "SEP_FIFO1_RXFD register accessor: an alias for `Reg<SEP_FIFO1_RXFD_SPEC>`"]
pub type SEP_FIFO1_RXFD = crate::Reg<sep_fifo1_rxfd::SEP_FIFO1_RXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo1_rxfd;
#[doc = "SEP_FIFO1_RXFDC_L register accessor: an alias for `Reg<SEP_FIFO1_RXFDC_L_SPEC>`"]
pub type SEP_FIFO1_RXFDC_L = crate::Reg<sep_fifo1_rxfdc_l::SEP_FIFO1_RXFDC_L_SPEC>;
#[doc = ""]
pub mod sep_fifo1_rxfdc_l;
#[doc = "SEP_FIFO1_RXFDC_H register accessor: an alias for `Reg<SEP_FIFO1_RXFDC_H_SPEC>`"]
pub type SEP_FIFO1_RXFDC_H = crate::Reg<sep_fifo1_rxfdc_h::SEP_FIFO1_RXFDC_H_SPEC>;
#[doc = ""]
pub mod sep_fifo1_rxfdc_h;
#[doc = "SEP_FIFO1_RXFC register accessor: an alias for `Reg<SEP_FIFO1_RXFC_SPEC>`"]
pub type SEP_FIFO1_RXFC = crate::Reg<sep_fifo1_rxfc::SEP_FIFO1_RXFC_SPEC>;
#[doc = ""]
pub mod sep_fifo1_rxfc;
#[doc = "SEP_FIFO1_TXFD register accessor: an alias for `Reg<SEP_FIFO1_TXFD_SPEC>`"]
pub type SEP_FIFO1_TXFD = crate::Reg<sep_fifo1_txfd::SEP_FIFO1_TXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo1_txfd;
#[doc = "SEP_FIFO1_TXFDC register accessor: an alias for `Reg<SEP_FIFO1_TXFDC_SPEC>`"]
pub type SEP_FIFO1_TXFDC = crate::Reg<sep_fifo1_txfdc::SEP_FIFO1_TXFDC_SPEC>;
#[doc = ""]
pub mod sep_fifo1_txfdc;
#[doc = "SEP_FIFO2_RXFD register accessor: an alias for `Reg<SEP_FIFO2_RXFD_SPEC>`"]
pub type SEP_FIFO2_RXFD = crate::Reg<sep_fifo2_rxfd::SEP_FIFO2_RXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo2_rxfd;
#[doc = "SEP_FIFO2_RXFDC_L register accessor: an alias for `Reg<SEP_FIFO2_RXFDC_L_SPEC>`"]
pub type SEP_FIFO2_RXFDC_L = crate::Reg<sep_fifo2_rxfdc_l::SEP_FIFO2_RXFDC_L_SPEC>;
#[doc = ""]
pub mod sep_fifo2_rxfdc_l;
#[doc = "SEP_FIFO2_RXFDC_H register accessor: an alias for `Reg<SEP_FIFO2_RXFDC_H_SPEC>`"]
pub type SEP_FIFO2_RXFDC_H = crate::Reg<sep_fifo2_rxfdc_h::SEP_FIFO2_RXFDC_H_SPEC>;
#[doc = ""]
pub mod sep_fifo2_rxfdc_h;
#[doc = "SEP_FIFO2_RXFC register accessor: an alias for `Reg<SEP_FIFO2_RXFC_SPEC>`"]
pub type SEP_FIFO2_RXFC = crate::Reg<sep_fifo2_rxfc::SEP_FIFO2_RXFC_SPEC>;
#[doc = ""]
pub mod sep_fifo2_rxfc;
#[doc = "SEP_FIFO2_TXFD register accessor: an alias for `Reg<SEP_FIFO2_TXFD_SPEC>`"]
pub type SEP_FIFO2_TXFD = crate::Reg<sep_fifo2_txfd::SEP_FIFO2_TXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo2_txfd;
#[doc = "SEP_FIFO2_TXFDC register accessor: an alias for `Reg<SEP_FIFO2_TXFDC_SPEC>`"]
pub type SEP_FIFO2_TXFDC = crate::Reg<sep_fifo2_txfdc::SEP_FIFO2_TXFDC_SPEC>;
#[doc = ""]
pub mod sep_fifo2_txfdc;
#[doc = "SEP_FIFO3_RXFD register accessor: an alias for `Reg<SEP_FIFO3_RXFD_SPEC>`"]
pub type SEP_FIFO3_RXFD = crate::Reg<sep_fifo3_rxfd::SEP_FIFO3_RXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo3_rxfd;
#[doc = "SEP_FIFO3_RXFDC_L register accessor: an alias for `Reg<SEP_FIFO3_RXFDC_L_SPEC>`"]
pub type SEP_FIFO3_RXFDC_L = crate::Reg<sep_fifo3_rxfdc_l::SEP_FIFO3_RXFDC_L_SPEC>;
#[doc = ""]
pub mod sep_fifo3_rxfdc_l;
#[doc = "SEP_FIFO3_RXFDC_H register accessor: an alias for `Reg<SEP_FIFO3_RXFDC_H_SPEC>`"]
pub type SEP_FIFO3_RXFDC_H = crate::Reg<sep_fifo3_rxfdc_h::SEP_FIFO3_RXFDC_H_SPEC>;
#[doc = ""]
pub mod sep_fifo3_rxfdc_h;
#[doc = "SEP_FIFO3_RXFC register accessor: an alias for `Reg<SEP_FIFO3_RXFC_SPEC>`"]
pub type SEP_FIFO3_RXFC = crate::Reg<sep_fifo3_rxfc::SEP_FIFO3_RXFC_SPEC>;
#[doc = ""]
pub mod sep_fifo3_rxfc;
#[doc = "SEP_FIFO3_TXFD register accessor: an alias for `Reg<SEP_FIFO3_TXFD_SPEC>`"]
pub type SEP_FIFO3_TXFD = crate::Reg<sep_fifo3_txfd::SEP_FIFO3_TXFD_SPEC>;
#[doc = ""]
pub mod sep_fifo3_txfd;
#[doc = "SEP_FIFO3_TXFDC register accessor: an alias for `Reg<SEP_FIFO3_TXFDC_SPEC>`"]
pub type SEP_FIFO3_TXFDC = crate::Reg<sep_fifo3_txfdc::SEP_FIFO3_TXFDC_SPEC>;
#[doc = ""]
pub mod sep_fifo3_txfdc;
#[doc = "HSCR register accessor: an alias for `Reg<HSCR_SPEC>`"]
pub type HSCR = crate::Reg<hscr::HSCR_SPEC>;
#[doc = "USB_HSCR Register"]
pub mod hscr;
#[doc = "HSVR register accessor: an alias for `Reg<HSVR_SPEC>`"]
pub type HSVR = crate::Reg<hsvr::HSVR_SPEC>;
#[doc = "USB_HSVR Register"]
pub mod hsvr;
