#[doc = "Register `RTC_CS` reader"]
pub struct R(crate::R<RTC_CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CS` writer"]
pub struct W(crate::W<RTC_CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CS_SPEC>;
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
impl From<crate::W<RTC_CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OWF` reader - "]
pub struct OWF_R(crate::FieldReader<bool, bool>);
impl OWF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OWF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWF` writer - "]
pub struct OWF_W<'a> {
    w: &'a mut W,
}
impl<'a> OWF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SECF` reader - "]
pub struct SECF_R(crate::FieldReader<bool, bool>);
impl SECF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECF` writer - "]
pub struct SECF_W<'a> {
    w: &'a mut W,
}
impl<'a> SECF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ALRF` reader - "]
pub struct ALRF_R(crate::FieldReader<bool, bool>);
impl ALRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRF` writer - "]
pub struct ALRF_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `OWF_IE` reader - "]
pub struct OWF_IE_R(crate::FieldReader<bool, bool>);
impl OWF_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OWF_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OWF_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OWF_IE` writer - "]
pub struct OWF_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> OWF_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `SECF_IE` reader - "]
pub struct SECF_IE_R(crate::FieldReader<bool, bool>);
impl SECF_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECF_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECF_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECF_IE` writer - "]
pub struct SECF_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SECF_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `ALRF_IE` reader - "]
pub struct ALRF_IE_R(crate::FieldReader<bool, bool>);
impl ALRF_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALRF_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALRF_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALRF_IE` writer - "]
pub struct ALRF_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRF_IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `WEC` reader - "]
pub struct WEC_R(crate::FieldReader<bool, bool>);
impl WEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WEC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WEC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WEC` writer - "]
pub struct WEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn owf(&self) -> OWF_R {
        OWF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn secf(&self) -> SECF_R {
        SECF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alrf(&self) -> ALRF_R {
        ALRF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn owf_ie(&self) -> OWF_IE_R {
        OWF_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn secf_ie(&self) -> SECF_IE_R {
        SECF_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alrf_ie(&self) -> ALRF_IE_R {
        ALRF_IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wec(&self) -> WEC_R {
        WEC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn owf(&mut self) -> OWF_W {
        OWF_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn secf(&mut self) -> SECF_W {
        SECF_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn alrf(&mut self) -> ALRF_W {
        ALRF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn owf_ie(&mut self) -> OWF_IE_W {
        OWF_IE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn secf_ie(&mut self) -> SECF_IE_W {
        SECF_IE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn alrf_ie(&mut self) -> ALRF_IE_W {
        ALRF_IE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn wec(&mut self) -> WEC_W {
        WEC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Realtime clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cs](index.html) module"]
pub struct RTC_CS_SPEC;
impl crate::RegisterSpec for RTC_CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cs::R](R) reader structure"]
impl crate::Readable for RTC_CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cs::W](W) writer structure"]
impl crate::Writable for RTC_CS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CS to value 0"]
impl crate::Resettable for RTC_CS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
