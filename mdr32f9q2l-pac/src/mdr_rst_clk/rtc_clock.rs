#[doc = "Register `RTC_CLOCK` reader"]
pub struct R(crate::R<RTC_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CLOCK` writer"]
pub struct W(crate::W<RTC_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<RTC_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE_SEL` reader - "]
pub struct HSE_SEL_R(crate::FieldReader<u8, u8>);
impl HSE_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE_SEL` writer - "]
pub struct HSE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `HSI_SEL` reader - "]
pub struct HSI_SEL_R(crate::FieldReader<u8, u8>);
impl HSI_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI_SEL` writer - "]
pub struct HSI_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `HSE_RTC_EN` reader - "]
pub struct HSE_RTC_EN_R(crate::FieldReader<bool, bool>);
impl HSE_RTC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSE_RTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_RTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE_RTC_EN` writer - "]
pub struct HSE_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_RTC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `HSI_RTC_EN` reader - "]
pub struct HSI_RTC_EN_R(crate::FieldReader<bool, bool>);
impl HSI_RTC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI_RTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI_RTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI_RTC_EN` writer - "]
pub struct HSI_RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI_RTC_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hse_sel(&self) -> HSE_SEL_R {
        HSE_SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsi_sel(&self) -> HSI_SEL_R {
        HSI_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hse_rtc_en(&self) -> HSE_RTC_EN_R {
        HSE_RTC_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hsi_rtc_en(&self) -> HSI_RTC_EN_R {
        HSI_RTC_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn hse_sel(&mut self) -> HSE_SEL_W {
        HSE_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hsi_sel(&mut self) -> HSI_SEL_W {
        HSI_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn hse_rtc_en(&mut self) -> HSE_RTC_EN_W {
        HSE_RTC_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn hsi_rtc_en(&mut self) -> HSI_RTC_EN_W {
        HSI_RTC_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_clock](index.html) module"]
pub struct RTC_CLOCK_SPEC;
impl crate::RegisterSpec for RTC_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_clock::R](R) reader structure"]
impl crate::Readable for RTC_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_clock::W](W) writer structure"]
impl crate::Writable for RTC_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CLOCK to value 0"]
impl crate::Resettable for RTC_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
