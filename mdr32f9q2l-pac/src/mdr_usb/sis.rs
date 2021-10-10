#[doc = "Register `SIS` reader"]
pub struct R(crate::R<SIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIS` writer"]
pub struct W(crate::W<SIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIS_SPEC>;
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
impl From<crate::W<SIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCTDONE` reader - "]
pub struct SCTDONE_R(crate::FieldReader<bool, bool>);
impl SCTDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCTDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCTDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCTDONE` writer - "]
pub struct SCTDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> SCTDONE_W<'a> {
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
#[doc = "Field `SCRESUME` reader - "]
pub struct SCRESUME_R(crate::FieldReader<bool, bool>);
impl SCRESUME_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRESUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRESUME_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRESUME` writer - "]
pub struct SCRESUME_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRESUME_W<'a> {
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
#[doc = "Field `SCRESETEV` reader - "]
pub struct SCRESETEV_R(crate::FieldReader<bool, bool>);
impl SCRESETEV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRESETEV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRESETEV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRESETEV` writer - "]
pub struct SCRESETEV_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRESETEV_W<'a> {
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
#[doc = "Field `SCSOFREC` reader - "]
pub struct SCSOFREC_R(crate::FieldReader<bool, bool>);
impl SCSOFREC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCSOFREC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCSOFREC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCSOFREC` writer - "]
pub struct SCSOFREC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSOFREC_W<'a> {
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
#[doc = "Field `SCNAKSENT` reader - "]
pub struct SCNAKSENT_R(crate::FieldReader<bool, bool>);
impl SCNAKSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCNAKSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCNAKSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCNAKSENT` writer - "]
pub struct SCNAKSENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCNAKSENT_W<'a> {
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
    pub fn sctdone(&self) -> SCTDONE_R {
        SCTDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresume(&self) -> SCRESUME_R {
        SCRESUME_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetev(&self) -> SCRESETEV_R {
        SCRESETEV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrec(&self) -> SCSOFREC_R {
        SCSOFREC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&self) -> SCNAKSENT_R {
        SCNAKSENT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sctdone(&mut self) -> SCTDONE_W {
        SCTDONE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scresume(&mut self) -> SCRESUME_W {
        SCRESUME_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scresetev(&mut self) -> SCRESETEV_W {
        SCRESETEV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scsofrec(&mut self) -> SCSOFREC_W {
        SCSOFREC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&mut self) -> SCNAKSENT_W {
        SCNAKSENT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SIS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sis](index.html) module"]
pub struct SIS_SPEC;
impl crate::RegisterSpec for SIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sis::R](R) reader structure"]
impl crate::Readable for SIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sis::W](W) writer structure"]
impl crate::Writable for SIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIS to value 0"]
impl crate::Resettable for SIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
