#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCGEN` reader - "]
pub struct SCGEN_R(crate::FieldReader<bool, bool>);
impl SCGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCGEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCGEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCGEN` writer - "]
pub struct SCGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SCGEN_W<'a> {
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
#[doc = "Field `SCTXLS` reader - "]
pub struct SCTXLS_R(crate::FieldReader<u8, u8>);
impl SCTXLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCTXLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCTXLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTXLS` writer - "]
pub struct SCTXLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTXLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `SCDC` reader - "]
pub struct SCDC_R(crate::FieldReader<bool, bool>);
impl SCDC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDC` writer - "]
pub struct SCDC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDC_W<'a> {
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
#[doc = "Field `SCFSP` reader - "]
pub struct SCFSP_R(crate::FieldReader<bool, bool>);
impl SCFSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCFSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCFSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCFSP` writer - "]
pub struct SCFSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFSP_W<'a> {
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
#[doc = "Field `SCFSR` reader - "]
pub struct SCFSR_R(crate::FieldReader<bool, bool>);
impl SCFSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCFSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCFSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCFSR` writer - "]
pub struct SCFSR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCFSR_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scgen(&self) -> SCGEN_R {
        SCGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sctxls(&self) -> SCTXLS_R {
        SCTXLS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scdc(&self) -> SCDC_R {
        SCDC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scfsp(&self) -> SCFSP_R {
        SCFSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scfsr(&self) -> SCFSR_R {
        SCFSR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn scgen(&mut self) -> SCGEN_W {
        SCGEN_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn sctxls(&mut self) -> SCTXLS_W {
        SCTXLS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scdc(&mut self) -> SCDC_W {
        SCDC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scfsp(&mut self) -> SCFSP_W {
        SCFSP_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scfsr(&mut self) -> SCFSR_W {
        SCFSR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
