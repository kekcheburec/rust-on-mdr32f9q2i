#[doc = "Register `SIM` reader"]
pub struct R(crate::R<SIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIM` writer"]
pub struct W(crate::W<SIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIM_SPEC>;
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
impl From<crate::W<SIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTDONEIE` reader - "]
pub struct SCTDONEIE_R(crate::FieldReader<bool, bool>);
impl SCTDONEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTDONEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCTDONEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTDONEIE` writer - "]
pub struct SCTDONEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTDONEIE_W<'a> {
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
#[doc = "Field `SCRESUMEIE` reader - "]
pub struct SCRESUMEIE_R(crate::FieldReader<bool, bool>);
impl SCRESUMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRESUMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRESUMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRESUMEIE` writer - "]
pub struct SCRESUMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRESUMEIE_W<'a> {
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
#[doc = "Field `SCRESETEVIE` reader - "]
pub struct SCRESETEVIE_R(crate::FieldReader<bool, bool>);
impl SCRESETEVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRESETEVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRESETEVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRESETEVIE` writer - "]
pub struct SCRESETEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRESETEVIE_W<'a> {
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
#[doc = "Field `SCSOFRECIE` reader - "]
pub struct SCSOFRECIE_R(crate::FieldReader<bool, bool>);
impl SCSOFRECIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCSOFRECIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCSOFRECIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCSOFRECIE` writer - "]
pub struct SCSOFRECIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSOFRECIE_W<'a> {
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
#[doc = "Field `SCNAKSENTIE` reader - "]
pub struct SCNAKSENTIE_R(crate::FieldReader<bool, bool>);
impl SCNAKSENTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCNAKSENTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCNAKSENTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCNAKSENTIE` writer - "]
pub struct SCNAKSENTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNAKSENTIE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sctdoneie(&self) -> SCTDONEIE_R {
        SCTDONEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresumeie(&self) -> SCRESUMEIE_R {
        SCRESUMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetevie(&self) -> SCRESETEVIE_R {
        SCRESETEVIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrecie(&self) -> SCSOFRECIE_R {
        SCSOFRECIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksentie(&self) -> SCNAKSENTIE_R {
        SCNAKSENTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sctdoneie(&mut self) -> SCTDONEIE_W {
        SCTDONEIE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresumeie(&mut self) -> SCRESUMEIE_W {
        SCRESUMEIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetevie(&mut self) -> SCRESETEVIE_W {
        SCRESETEVIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrecie(&mut self) -> SCSOFRECIE_W {
        SCSOFRECIE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksentie(&mut self) -> SCNAKSENTIE_W {
        SCNAKSENTIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SIM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sim](index.html) module"]
pub struct SIM_SPEC;
impl crate::RegisterSpec for SIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sim::R](R) reader structure"]
impl crate::Readable for SIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sim::W](W) writer structure"]
impl crate::Writable for SIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIM to value 0"]
impl crate::Resettable for SIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
