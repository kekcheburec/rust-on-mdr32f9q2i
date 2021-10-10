#[doc = "Register `HTXC` reader"]
pub struct R(crate::R<HTXC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXC` writer"]
pub struct W(crate::W<HTXC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXC_SPEC>;
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
impl From<crate::W<HTXC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TREQ` reader - "]
pub struct TREQ_R(crate::FieldReader<bool, bool>);
impl TREQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TREQ` writer - "]
pub struct TREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TREQ_W<'a> {
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
#[doc = "Field `SOFS` reader - "]
pub struct SOFS_R(crate::FieldReader<bool, bool>);
impl SOFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFS` writer - "]
pub struct SOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFS_W<'a> {
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
#[doc = "Field `PREEN` reader - "]
pub struct PREEN_R(crate::FieldReader<bool, bool>);
impl PREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PREEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PREEN` writer - "]
pub struct PREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREEN_W<'a> {
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
#[doc = "Field `ISOEN` reader - "]
pub struct ISOEN_R(crate::FieldReader<bool, bool>);
impl ISOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ISOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOEN` writer - "]
pub struct ISOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOEN_W<'a> {
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
    pub fn treq(&self) -> TREQ_R {
        TREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sofs(&self) -> SOFS_R {
        SOFS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn isoen(&self) -> ISOEN_R {
        ISOEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn treq(&mut self) -> TREQ_W {
        TREQ_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sofs(&mut self) -> SOFS_W {
        SOFS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W {
        PREEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn isoen(&mut self) -> ISOEN_W {
        ISOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxc](index.html) module"]
pub struct HTXC_SPEC;
impl crate::RegisterSpec for HTXC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxc::R](R) reader structure"]
impl crate::Readable for HTXC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxc::W](W) writer structure"]
impl crate::Writable for HTXC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXC to value 0"]
impl crate::Resettable for HTXC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
