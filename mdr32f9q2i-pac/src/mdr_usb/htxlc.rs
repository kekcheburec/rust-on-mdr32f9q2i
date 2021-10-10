#[doc = "Register `HTXLC` reader"]
pub struct R(crate::R<HTXLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXLC` writer"]
pub struct W(crate::W<HTXLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXLC_SPEC>;
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
impl From<crate::W<HTXLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXLC` reader - "]
pub struct TXLC_R(crate::FieldReader<u8, u8>);
impl TXLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXLC` writer - "]
pub struct TXLC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `DC` reader - "]
pub struct DC_R(crate::FieldReader<bool, bool>);
impl DC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DC` writer - "]
pub struct DC_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_W<'a> {
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
#[doc = "Field `FSPL` reader - "]
pub struct FSPL_R(crate::FieldReader<bool, bool>);
impl FSPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSPL` writer - "]
pub struct FSPL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSPL_W<'a> {
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
#[doc = "Field `FSLR` reader - "]
pub struct FSLR_R(crate::FieldReader<bool, bool>);
impl FSLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLR` writer - "]
pub struct FSLR_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLR_W<'a> {
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
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txlc(&self) -> TXLC_R {
        TXLC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fspl(&self) -> FSPL_R {
        FSPL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fslr(&self) -> FSLR_R {
        FSLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn txlc(&mut self) -> TXLC_W {
        TXLC_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dc(&mut self) -> DC_W {
        DC_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fspl(&mut self) -> FSPL_W {
        FSPL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fslr(&mut self) -> FSLR_W {
        FSLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXLC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxlc](index.html) module"]
pub struct HTXLC_SPEC;
impl crate::RegisterSpec for HTXLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxlc::R](R) reader structure"]
impl crate::Readable for HTXLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxlc::W](W) writer structure"]
impl crate::Writable for HTXLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXLC to value 0"]
impl crate::Resettable for HTXLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
