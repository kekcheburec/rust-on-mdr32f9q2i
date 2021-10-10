#[doc = "Register `HRXS` reader"]
pub struct R(crate::R<HRXS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXS` writer"]
pub struct W(crate::W<HRXS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXS_SPEC>;
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
impl From<crate::W<HRXS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCERR` reader - "]
pub struct CRCERR_R(crate::FieldReader<bool, bool>);
impl CRCERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCERR` writer - "]
pub struct CRCERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCERR_W<'a> {
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
#[doc = "Field `BSERR` reader - "]
pub struct BSERR_R(crate::FieldReader<bool, bool>);
impl BSERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BSERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSERR` writer - "]
pub struct BSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BSERR_W<'a> {
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
#[doc = "Field `RXOF` reader - "]
pub struct RXOF_R(crate::FieldReader<bool, bool>);
impl RXOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOF` writer - "]
pub struct RXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOF_W<'a> {
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
#[doc = "Field `RXTO` reader - "]
pub struct RXTO_R(crate::FieldReader<bool, bool>);
impl RXTO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTO` writer - "]
pub struct RXTO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTO_W<'a> {
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
#[doc = "Field `NAKRXED` reader - "]
pub struct NAKRXED_R(crate::FieldReader<bool, bool>);
impl NAKRXED_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKRXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKRXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKRXED` writer - "]
pub struct NAKRXED_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKRXED_W<'a> {
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
#[doc = "Field `STALLRXED` reader - "]
pub struct STALLRXED_R(crate::FieldReader<bool, bool>);
impl STALLRXED_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLRXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLRXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLRXED` writer - "]
pub struct STALLRXED_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRXED_W<'a> {
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
#[doc = "Field `ACKRXED` reader - "]
pub struct ACKRXED_R(crate::FieldReader<bool, bool>);
impl ACKRXED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACKRXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACKRXED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACKRXED` writer - "]
pub struct ACKRXED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKRXED_W<'a> {
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
#[doc = "Field `DATASEQ` reader - "]
pub struct DATASEQ_R(crate::FieldReader<bool, bool>);
impl DATASEQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATASEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATASEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATASEQ` writer - "]
pub struct DATASEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DATASEQ_W<'a> {
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
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bserr(&self) -> BSERR_R {
        BSERR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nakrxed(&self) -> NAKRXED_R {
        NAKRXED_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stallrxed(&self) -> STALLRXED_R {
        STALLRXED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ackrxed(&self) -> ACKRXED_R {
        ACKRXED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dataseq(&self) -> DATASEQ_R {
        DATASEQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn crcerr(&mut self) -> CRCERR_W {
        CRCERR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bserr(&mut self) -> BSERR_W {
        BSERR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RXTO_W {
        RXTO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn nakrxed(&mut self) -> NAKRXED_W {
        NAKRXED_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn stallrxed(&mut self) -> STALLRXED_W {
        STALLRXED_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ackrxed(&mut self) -> ACKRXED_W {
        ACKRXED_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dataseq(&mut self) -> DATASEQ_W {
        DATASEQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HRXS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxs](index.html) module"]
pub struct HRXS_SPEC;
impl crate::RegisterSpec for HRXS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxs::R](R) reader structure"]
impl crate::Readable for HRXS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxs::W](W) writer structure"]
impl crate::Writable for HRXS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRXS to value 0"]
impl crate::Resettable for HRXS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
