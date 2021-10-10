#[doc = "Register `CLOCK_STATUS` reader"]
pub struct R(crate::R<CLOCK_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCK_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCK_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCK_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLL_USB_RDY` reader - "]
pub struct PLL_USB_RDY_R(crate::FieldReader<bool, bool>);
impl PLL_USB_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_USB_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_USB_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CPU_RDY` reader - "]
pub struct PLL_CPU_RDY_R(crate::FieldReader<bool, bool>);
impl PLL_CPU_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_CPU_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CPU_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE_RDY` reader - "]
pub struct HSE_RDY_R(crate::FieldReader<bool, bool>);
impl HSE_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSE_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pll_usb_rdy(&self) -> PLL_USB_RDY_R {
        PLL_USB_RDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pll_cpu_rdy(&self) -> PLL_CPU_RDY_R {
        PLL_CPU_RDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn hse_rdy(&self) -> HSE_RDY_R {
        HSE_RDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clock_status](index.html) module"]
pub struct CLOCK_STATUS_SPEC;
impl crate::RegisterSpec for CLOCK_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clock_status::R](R) reader structure"]
impl crate::Readable for CLOCK_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLOCK_STATUS to value 0"]
impl crate::Resettable for CLOCK_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
