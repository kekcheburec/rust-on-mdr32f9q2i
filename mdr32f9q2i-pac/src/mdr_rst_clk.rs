#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Clock Status Register"]
    pub clock_status: crate::Reg<clock_status::CLOCK_STATUS_SPEC>,
    #[doc = "0x04 - PLL Control Register"]
    pub pll_control: crate::Reg<pll_control::PLL_CONTROL_SPEC>,
    #[doc = "0x08 - HS Control Register"]
    pub hs_control: crate::Reg<hs_control::HS_CONTROL_SPEC>,
    #[doc = "0x0c - CPU Clock Register"]
    pub cpu_clock: crate::Reg<cpu_clock::CPU_CLOCK_SPEC>,
    #[doc = "0x10 - USB Clock Register"]
    pub usb_clock: crate::Reg<usb_clock::USB_CLOCK_SPEC>,
    #[doc = "0x14 - ADC Clock Register"]
    pub adc_mco_clock: crate::Reg<adc_mco_clock::ADC_MCO_CLOCK_SPEC>,
    #[doc = "0x18 - RTC Clock Register"]
    pub rtc_clock: crate::Reg<rtc_clock::RTC_CLOCK_SPEC>,
    #[doc = "0x1c - Peripheral Clock Enable Register"]
    pub per_clock: crate::Reg<per_clock::PER_CLOCK_SPEC>,
    #[doc = "0x20 - CAN Clock Register"]
    pub can_clock: crate::Reg<can_clock::CAN_CLOCK_SPEC>,
    #[doc = "0x24 - Timer Clock Register"]
    pub tim_clock: crate::Reg<tim_clock::TIM_CLOCK_SPEC>,
    #[doc = "0x28 - UART Clock Register"]
    pub uart_clock: crate::Reg<uart_clock::UART_CLOCK_SPEC>,
    #[doc = "0x2c - SSP Clock Register"]
    pub ssp_clock: crate::Reg<ssp_clock::SSP_CLOCK_SPEC>,
}
#[doc = "CLOCK_STATUS register accessor: an alias for `Reg<CLOCK_STATUS_SPEC>`"]
pub type CLOCK_STATUS = crate::Reg<clock_status::CLOCK_STATUS_SPEC>;
#[doc = "Clock Status Register"]
pub mod clock_status;
#[doc = "PLL_CONTROL register accessor: an alias for `Reg<PLL_CONTROL_SPEC>`"]
pub type PLL_CONTROL = crate::Reg<pll_control::PLL_CONTROL_SPEC>;
#[doc = "PLL Control Register"]
pub mod pll_control;
#[doc = "HS_CONTROL register accessor: an alias for `Reg<HS_CONTROL_SPEC>`"]
pub type HS_CONTROL = crate::Reg<hs_control::HS_CONTROL_SPEC>;
#[doc = "HS Control Register"]
pub mod hs_control;
#[doc = "CPU_CLOCK register accessor: an alias for `Reg<CPU_CLOCK_SPEC>`"]
pub type CPU_CLOCK = crate::Reg<cpu_clock::CPU_CLOCK_SPEC>;
#[doc = "CPU Clock Register"]
pub mod cpu_clock;
#[doc = "USB_CLOCK register accessor: an alias for `Reg<USB_CLOCK_SPEC>`"]
pub type USB_CLOCK = crate::Reg<usb_clock::USB_CLOCK_SPEC>;
#[doc = "USB Clock Register"]
pub mod usb_clock;
#[doc = "ADC_MCO_CLOCK register accessor: an alias for `Reg<ADC_MCO_CLOCK_SPEC>`"]
pub type ADC_MCO_CLOCK = crate::Reg<adc_mco_clock::ADC_MCO_CLOCK_SPEC>;
#[doc = "ADC Clock Register"]
pub mod adc_mco_clock;
#[doc = "RTC_CLOCK register accessor: an alias for `Reg<RTC_CLOCK_SPEC>`"]
pub type RTC_CLOCK = crate::Reg<rtc_clock::RTC_CLOCK_SPEC>;
#[doc = "RTC Clock Register"]
pub mod rtc_clock;
#[doc = "PER_CLOCK register accessor: an alias for `Reg<PER_CLOCK_SPEC>`"]
pub type PER_CLOCK = crate::Reg<per_clock::PER_CLOCK_SPEC>;
#[doc = "Peripheral Clock Enable Register"]
pub mod per_clock;
#[doc = "CAN_CLOCK register accessor: an alias for `Reg<CAN_CLOCK_SPEC>`"]
pub type CAN_CLOCK = crate::Reg<can_clock::CAN_CLOCK_SPEC>;
#[doc = "CAN Clock Register"]
pub mod can_clock;
#[doc = "TIM_CLOCK register accessor: an alias for `Reg<TIM_CLOCK_SPEC>`"]
pub type TIM_CLOCK = crate::Reg<tim_clock::TIM_CLOCK_SPEC>;
#[doc = "Timer Clock Register"]
pub mod tim_clock;
#[doc = "UART_CLOCK register accessor: an alias for `Reg<UART_CLOCK_SPEC>`"]
pub type UART_CLOCK = crate::Reg<uart_clock::UART_CLOCK_SPEC>;
#[doc = "UART Clock Register"]
pub mod uart_clock;
#[doc = "SSP_CLOCK register accessor: an alias for `Reg<SSP_CLOCK_SPEC>`"]
pub type SSP_CLOCK = crate::Reg<ssp_clock::SSP_CLOCK_SPEC>;
#[doc = "SSP Clock Register"]
pub mod ssp_clock;
