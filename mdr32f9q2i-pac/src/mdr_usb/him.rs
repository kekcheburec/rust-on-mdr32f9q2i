#[doc = "Register `HIM` reader"]
pub struct R(crate::R<HIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIM` writer"]
pub struct W(crate::W<HIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIM_SPEC>;
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
impl From<crate::W<HIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDONEIE` reader - "]
pub struct TDONEIE_R(crate::FieldReader<bool, bool>);
impl TDONEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDONEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDONEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDONEIE` writer - "]
pub struct TDONEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDONEIE_W<'a> {
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
#[doc = "Field `RESUMEIE` reader - "]
pub struct RESUMEIE_R(crate::FieldReader<bool, bool>);
impl RESUMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUMEIE` writer - "]
pub struct RESUMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUMEIE_W<'a> {
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
#[doc = "Field `CONEVIE` reader - "]
pub struct CONEVIE_R(crate::FieldReader<bool, bool>);
impl CONEVIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONEVIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONEVIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONEVIE` writer - "]
pub struct CONEVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CONEVIE_W<'a> {
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
#[doc = "Field `SOFIE` reader - "]
pub struct SOFIE_R(crate::FieldReader<bool, bool>);
impl SOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFIE` writer - "]
pub struct SOFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFIE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoneie(&self) -> TDONEIE_R {
        TDONEIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conevie(&self) -> CONEVIE_R {
        CONEVIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdoneie(&mut self) -> TDONEIE_W {
        TDONEIE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resumeie(&mut self) -> RESUMEIE_W {
        RESUMEIE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conevie(&mut self) -> CONEVIE_W {
        CONEVIE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofie(&mut self) -> SOFIE_W {
        SOFIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HIM Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [him](index.html) module"]
pub struct HIM_SPEC;
impl crate::RegisterSpec for HIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [him::R](R) reader structure"]
impl crate::Readable for HIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [him::W](W) writer structure"]
impl crate::Writable for HIM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HIM to value 0"]
impl crate::Resettable for HIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
