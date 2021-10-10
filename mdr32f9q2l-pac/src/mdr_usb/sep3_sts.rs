#[doc = "Register `SEP3_STS` reader"]
pub struct R(crate::R<SEP3_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP3_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP3_STS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP3_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP3_STS` writer"]
pub struct W(crate::W<SEP3_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP3_STS_SPEC>;
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
impl From<crate::W<SEP3_STS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP3_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCCRCERR` reader - "]
pub struct SCCRCERR_R(crate::FieldReader<bool, bool>);
impl SCCRCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCCRCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCCRCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCCRCERR` writer - "]
pub struct SCCRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCCRCERR_W<'a> {
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
#[doc = "Field `SCBSERR` reader - "]
pub struct SCBSERR_R(crate::FieldReader<bool, bool>);
impl SCBSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCBSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCBSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCBSERR` writer - "]
pub struct SCBSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCBSERR_W<'a> {
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
#[doc = "Field `SCRXOF` reader - "]
pub struct SCRXOF_R(crate::FieldReader<bool, bool>);
impl SCRXOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRXOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRXOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRXOF` writer - "]
pub struct SCRXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRXOF_W<'a> {
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
#[doc = "Field `SCRXTO` reader - "]
pub struct SCRXTO_R(crate::FieldReader<bool, bool>);
impl SCRXTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRXTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRXTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRXTO` writer - "]
pub struct SCRXTO_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRXTO_W<'a> {
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
#[doc = "Field `SCSTALLSENT` reader - "]
pub struct SCSTALLSENT_R(crate::FieldReader<bool, bool>);
impl SCSTALLSENT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCSTALLSENT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCSTALLSENT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCSTALLSENT` writer - "]
pub struct SCSTALLSENT_W<'a> {
    w: &'a mut W,
}
impl<'a> SCSTALLSENT_W<'a> {
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
#[doc = "Field `SCACKRXED` reader - "]
pub struct SCACKRXED_R(crate::FieldReader<bool, bool>);
impl SCACKRXED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCACKRXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCACKRXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCACKRXED` writer - "]
pub struct SCACKRXED_W<'a> {
    w: &'a mut W,
}
impl<'a> SCACKRXED_W<'a> {
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
#[doc = "Field `SCDATASEQ` reader - "]
pub struct SCDATASEQ_R(crate::FieldReader<bool, bool>);
impl SCDATASEQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCDATASEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDATASEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDATASEQ` writer - "]
pub struct SCDATASEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDATASEQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sccrcerr(&self) -> SCCRCERR_R {
        SCCRCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scbserr(&self) -> SCBSERR_R {
        SCBSERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scrxof(&self) -> SCRXOF_R {
        SCRXOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scrxto(&self) -> SCRXTO_R {
        SCRXTO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&self) -> SCNAKSENT_R {
        SCNAKSENT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scstallsent(&self) -> SCSTALLSENT_R {
        SCSTALLSENT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scackrxed(&self) -> SCACKRXED_R {
        SCACKRXED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn scdataseq(&self) -> SCDATASEQ_R {
        SCDATASEQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sccrcerr(&mut self) -> SCCRCERR_W {
        SCCRCERR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn scbserr(&mut self) -> SCBSERR_W {
        SCBSERR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn scrxof(&mut self) -> SCRXOF_W {
        SCRXOF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn scrxto(&mut self) -> SCRXTO_W {
        SCRXTO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn scnaksent(&mut self) -> SCNAKSENT_W {
        SCNAKSENT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn scstallsent(&mut self) -> SCSTALLSENT_W {
        SCSTALLSENT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn scackrxed(&mut self) -> SCACKRXED_W {
        SCACKRXED_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn scdataseq(&mut self) -> SCDATASEQ_W {
        SCDATASEQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SEP Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep3_sts](index.html) module"]
pub struct SEP3_STS_SPEC;
impl crate::RegisterSpec for SEP3_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep3_sts::R](R) reader structure"]
impl crate::Readable for SEP3_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep3_sts::W](W) writer structure"]
impl crate::Writable for SEP3_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP3_STS to value 0"]
impl crate::Resettable for SEP3_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
