#[doc = "Register `HIS` reader"]
pub struct R(crate::R<HIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HIS` writer"]
pub struct W(crate::W<HIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HIS_SPEC>;
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
impl From<crate::W<HIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDONE` reader - "]
pub struct TDONE_R(crate::FieldReader<bool, bool>);
impl TDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDONE` writer - "]
pub struct TDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDONE_W<'a> {
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
#[doc = "Field `RESUME` reader - "]
pub struct RESUME_R(crate::FieldReader<bool, bool>);
impl RESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESUME` writer - "]
pub struct RESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> RESUME_W<'a> {
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
#[doc = "Field `CONEV` reader - "]
pub struct CONEV_R(crate::FieldReader<bool, bool>);
impl CONEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONEV` writer - "]
pub struct CONEV_W<'a> {
    w: &'a mut W,
}
impl<'a> CONEV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdone(&self) -> TDONE_R {
        TDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conev(&self) -> CONEV_R {
        CONEV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofs(&self) -> SOFS_R {
        SOFS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdone(&mut self) -> TDONE_W {
        TDONE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn resume(&mut self) -> RESUME_W {
        RESUME_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn conev(&mut self) -> CONEV_W {
        CONEV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sofs(&mut self) -> SOFS_W {
        SOFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HIS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [his](index.html) module"]
pub struct HIS_SPEC;
impl crate::RegisterSpec for HIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [his::R](R) reader structure"]
impl crate::Readable for HIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [his::W](W) writer structure"]
impl crate::Writable for HIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HIS to value 0"]
impl crate::Resettable for HIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
