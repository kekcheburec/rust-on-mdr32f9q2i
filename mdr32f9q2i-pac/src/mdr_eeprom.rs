#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEPROM Command Register"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x04 - EEPROM Address Register"]
    pub adr: crate::Reg<adr::ADR_SPEC>,
    #[doc = "0x08 - EEPROM Read Data Register"]
    pub di: crate::Reg<di::DI_SPEC>,
    #[doc = "0x0c - EEPROM Write Data Register"]
    pub do_: crate::Reg<do_::DO_SPEC>,
    #[doc = "0x10 - EEPROM Key Register"]
    pub key: crate::Reg<key::KEY_SPEC>,
}
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "EEPROM Command Register"]
pub mod cmd;
#[doc = "ADR register accessor: an alias for `Reg<ADR_SPEC>`"]
pub type ADR = crate::Reg<adr::ADR_SPEC>;
#[doc = "EEPROM Address Register"]
pub mod adr;
#[doc = "DI register accessor: an alias for `Reg<DI_SPEC>`"]
pub type DI = crate::Reg<di::DI_SPEC>;
#[doc = "EEPROM Read Data Register"]
pub mod di;
#[doc = "DO register accessor: an alias for `Reg<DO_SPEC>`"]
pub type DO = crate::Reg<do_::DO_SPEC>;
#[doc = "EEPROM Write Data Register"]
pub mod do_;
#[doc = "KEY register accessor: an alias for `Reg<KEY_SPEC>`"]
pub type KEY = crate::Reg<key::KEY_SPEC>;
#[doc = "EEPROM Key Register"]
pub mod key;
