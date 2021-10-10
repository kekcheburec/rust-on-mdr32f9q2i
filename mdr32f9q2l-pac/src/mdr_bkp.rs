#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup Register 0"]
    pub reg_00: crate::Reg<reg_00::REG_00_SPEC>,
    #[doc = "0x04 - Backup Register 1"]
    pub reg_01: crate::Reg<reg_01::REG_01_SPEC>,
    #[doc = "0x08 - Backup Register 2"]
    pub reg_02: crate::Reg<reg_02::REG_02_SPEC>,
    #[doc = "0x0c - Backup Register 3"]
    pub reg_03: crate::Reg<reg_03::REG_03_SPEC>,
    #[doc = "0x10 - Backup Register 4"]
    pub reg_04: crate::Reg<reg_04::REG_04_SPEC>,
    #[doc = "0x14 - Backup Register 5"]
    pub reg_05: crate::Reg<reg_05::REG_05_SPEC>,
    #[doc = "0x18 - Backup Register 6"]
    pub reg_06: crate::Reg<reg_06::REG_06_SPEC>,
    #[doc = "0x1c - Backup Register 7"]
    pub reg_07: crate::Reg<reg_07::REG_07_SPEC>,
    #[doc = "0x20 - Backup Register 8"]
    pub reg_08: crate::Reg<reg_08::REG_08_SPEC>,
    #[doc = "0x24 - Backup Register 9"]
    pub reg_09: crate::Reg<reg_09::REG_09_SPEC>,
    #[doc = "0x28 - Backup Register 10"]
    pub reg_0a: crate::Reg<reg_0a::REG_0A_SPEC>,
    #[doc = "0x2c - Backup Register 11"]
    pub reg_0b: crate::Reg<reg_0b::REG_0B_SPEC>,
    #[doc = "0x30 - Backup Register 12"]
    pub reg_0c: crate::Reg<reg_0c::REG_0C_SPEC>,
    #[doc = "0x34 - Backup Register 13"]
    pub reg_0d: crate::Reg<reg_0d::REG_0D_SPEC>,
    #[doc = "0x38 - Backup Register 14"]
    pub reg_0e: crate::Reg<reg_0e::REG_0E_SPEC>,
    #[doc = "0x3c - Backup Register 15"]
    pub reg_0f: crate::Reg<reg_0f::REG_0F_SPEC>,
    #[doc = "0x40 - Realtime Clock Counter Register"]
    pub rtc_cnt: crate::Reg<rtc_cnt::RTC_CNT_SPEC>,
    #[doc = "0x44 - Realtime Prescaler Counter Register"]
    pub rtc_div: crate::Reg<rtc_div::RTC_DIV_SPEC>,
    #[doc = "0x48 - Realtime Prescaler Auto-Reload Value Register"]
    pub rtc_prl: crate::Reg<rtc_prl::RTC_PRL_SPEC>,
    #[doc = "0x4c - Realtime Alarm Register"]
    pub rtc_alrm: crate::Reg<rtc_alrm::RTC_ALRM_SPEC>,
    #[doc = "0x50 - Backup Realtime clock Register"]
    pub rtc_cs: crate::Reg<rtc_cs::RTC_CS_SPEC>,
}
#[doc = "REG_00 register accessor: an alias for `Reg<REG_00_SPEC>`"]
pub type REG_00 = crate::Reg<reg_00::REG_00_SPEC>;
#[doc = "Backup Register 0"]
pub mod reg_00;
#[doc = "REG_01 register accessor: an alias for `Reg<REG_01_SPEC>`"]
pub type REG_01 = crate::Reg<reg_01::REG_01_SPEC>;
#[doc = "Backup Register 1"]
pub mod reg_01;
#[doc = "REG_02 register accessor: an alias for `Reg<REG_02_SPEC>`"]
pub type REG_02 = crate::Reg<reg_02::REG_02_SPEC>;
#[doc = "Backup Register 2"]
pub mod reg_02;
#[doc = "REG_03 register accessor: an alias for `Reg<REG_03_SPEC>`"]
pub type REG_03 = crate::Reg<reg_03::REG_03_SPEC>;
#[doc = "Backup Register 3"]
pub mod reg_03;
#[doc = "REG_04 register accessor: an alias for `Reg<REG_04_SPEC>`"]
pub type REG_04 = crate::Reg<reg_04::REG_04_SPEC>;
#[doc = "Backup Register 4"]
pub mod reg_04;
#[doc = "REG_05 register accessor: an alias for `Reg<REG_05_SPEC>`"]
pub type REG_05 = crate::Reg<reg_05::REG_05_SPEC>;
#[doc = "Backup Register 5"]
pub mod reg_05;
#[doc = "REG_06 register accessor: an alias for `Reg<REG_06_SPEC>`"]
pub type REG_06 = crate::Reg<reg_06::REG_06_SPEC>;
#[doc = "Backup Register 6"]
pub mod reg_06;
#[doc = "REG_07 register accessor: an alias for `Reg<REG_07_SPEC>`"]
pub type REG_07 = crate::Reg<reg_07::REG_07_SPEC>;
#[doc = "Backup Register 7"]
pub mod reg_07;
#[doc = "REG_08 register accessor: an alias for `Reg<REG_08_SPEC>`"]
pub type REG_08 = crate::Reg<reg_08::REG_08_SPEC>;
#[doc = "Backup Register 8"]
pub mod reg_08;
#[doc = "REG_09 register accessor: an alias for `Reg<REG_09_SPEC>`"]
pub type REG_09 = crate::Reg<reg_09::REG_09_SPEC>;
#[doc = "Backup Register 9"]
pub mod reg_09;
#[doc = "REG_0A register accessor: an alias for `Reg<REG_0A_SPEC>`"]
pub type REG_0A = crate::Reg<reg_0a::REG_0A_SPEC>;
#[doc = "Backup Register 10"]
pub mod reg_0a;
#[doc = "REG_0B register accessor: an alias for `Reg<REG_0B_SPEC>`"]
pub type REG_0B = crate::Reg<reg_0b::REG_0B_SPEC>;
#[doc = "Backup Register 11"]
pub mod reg_0b;
#[doc = "REG_0C register accessor: an alias for `Reg<REG_0C_SPEC>`"]
pub type REG_0C = crate::Reg<reg_0c::REG_0C_SPEC>;
#[doc = "Backup Register 12"]
pub mod reg_0c;
#[doc = "REG_0D register accessor: an alias for `Reg<REG_0D_SPEC>`"]
pub type REG_0D = crate::Reg<reg_0d::REG_0D_SPEC>;
#[doc = "Backup Register 13"]
pub mod reg_0d;
#[doc = "REG_0E register accessor: an alias for `Reg<REG_0E_SPEC>`"]
pub type REG_0E = crate::Reg<reg_0e::REG_0E_SPEC>;
#[doc = "Backup Register 14"]
pub mod reg_0e;
#[doc = "REG_0F register accessor: an alias for `Reg<REG_0F_SPEC>`"]
pub type REG_0F = crate::Reg<reg_0f::REG_0F_SPEC>;
#[doc = "Backup Register 15"]
pub mod reg_0f;
#[doc = "RTC_CNT register accessor: an alias for `Reg<RTC_CNT_SPEC>`"]
pub type RTC_CNT = crate::Reg<rtc_cnt::RTC_CNT_SPEC>;
#[doc = "Realtime Clock Counter Register"]
pub mod rtc_cnt;
#[doc = "RTC_DIV register accessor: an alias for `Reg<RTC_DIV_SPEC>`"]
pub type RTC_DIV = crate::Reg<rtc_div::RTC_DIV_SPEC>;
#[doc = "Realtime Prescaler Counter Register"]
pub mod rtc_div;
#[doc = "RTC_PRL register accessor: an alias for `Reg<RTC_PRL_SPEC>`"]
pub type RTC_PRL = crate::Reg<rtc_prl::RTC_PRL_SPEC>;
#[doc = "Realtime Prescaler Auto-Reload Value Register"]
pub mod rtc_prl;
#[doc = "RTC_ALRM register accessor: an alias for `Reg<RTC_ALRM_SPEC>`"]
pub type RTC_ALRM = crate::Reg<rtc_alrm::RTC_ALRM_SPEC>;
#[doc = "Realtime Alarm Register"]
pub mod rtc_alrm;
#[doc = "RTC_CS register accessor: an alias for `Reg<RTC_CS_SPEC>`"]
pub type RTC_CS = crate::Reg<rtc_cs::RTC_CS_SPEC>;
#[doc = "Backup Realtime clock Register"]
pub mod rtc_cs;
