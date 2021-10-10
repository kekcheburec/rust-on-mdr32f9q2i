#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PORT Data Register"]
    pub rxtx: crate::Reg<rxtx::RXTX_SPEC>,
    #[doc = "0x04 - PORT Output Enable Register"]
    pub oe: crate::Reg<oe::OE_SPEC>,
    #[doc = "0x08 - PORT Function Register"]
    pub func: crate::Reg<func::FUNC_SPEC>,
    #[doc = "0x0c - PORT Analog Function Register"]
    pub analog: crate::Reg<analog::ANALOG_SPEC>,
    #[doc = "0x10 - PORT Pull Up/Down Register"]
    pub pull: crate::Reg<pull::PULL_SPEC>,
    #[doc = "0x14 - PORT Driver Mode Register"]
    pub pd: crate::Reg<pd::PD_SPEC>,
    #[doc = "0x18 - PORT Power Register"]
    pub pwr: crate::Reg<pwr::PWR_SPEC>,
    #[doc = "0x1c - PORT Filter Configuration Register"]
    pub gfen: crate::Reg<gfen::GFEN_SPEC>,
}
#[doc = "RXTX register accessor: an alias for `Reg<RXTX_SPEC>`"]
pub type RXTX = crate::Reg<rxtx::RXTX_SPEC>;
#[doc = "PORT Data Register"]
pub mod rxtx;
#[doc = "OE register accessor: an alias for `Reg<OE_SPEC>`"]
pub type OE = crate::Reg<oe::OE_SPEC>;
#[doc = "PORT Output Enable Register"]
pub mod oe;
#[doc = "FUNC register accessor: an alias for `Reg<FUNC_SPEC>`"]
pub type FUNC = crate::Reg<func::FUNC_SPEC>;
#[doc = "PORT Function Register"]
pub mod func;
#[doc = "ANALOG register accessor: an alias for `Reg<ANALOG_SPEC>`"]
pub type ANALOG = crate::Reg<analog::ANALOG_SPEC>;
#[doc = "PORT Analog Function Register"]
pub mod analog;
#[doc = "PULL register accessor: an alias for `Reg<PULL_SPEC>`"]
pub type PULL = crate::Reg<pull::PULL_SPEC>;
#[doc = "PORT Pull Up/Down Register"]
pub mod pull;
#[doc = "PD register accessor: an alias for `Reg<PD_SPEC>`"]
pub type PD = crate::Reg<pd::PD_SPEC>;
#[doc = "PORT Driver Mode Register"]
pub mod pd;
#[doc = "PWR register accessor: an alias for `Reg<PWR_SPEC>`"]
pub type PWR = crate::Reg<pwr::PWR_SPEC>;
#[doc = "PORT Power Register"]
pub mod pwr;
#[doc = "GFEN register accessor: an alias for `Reg<GFEN_SPEC>`"]
pub type GFEN = crate::Reg<gfen::GFEN_SPEC>;
#[doc = "PORT Filter Configuration Register"]
pub mod gfen;
