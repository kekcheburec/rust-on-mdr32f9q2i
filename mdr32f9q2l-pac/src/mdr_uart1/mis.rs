#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIS` writer"]
pub struct W(crate::W<MIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIS_SPEC>;
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
impl From<crate::W<MIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIMMIS` reader - "]
pub struct RIMMIS_R(crate::FieldReader<bool, bool>);
impl RIMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIMMIS` writer - "]
pub struct RIMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMMIS_W<'a> {
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
#[doc = "Field `CTSMMIS` reader - "]
pub struct CTSMMIS_R(crate::FieldReader<bool, bool>);
impl CTSMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMMIS` writer - "]
pub struct CTSMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMMIS_W<'a> {
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
#[doc = "Field `DCDMMIS` reader - "]
pub struct DCDMMIS_R(crate::FieldReader<bool, bool>);
impl DCDMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMMIS` writer - "]
pub struct DCDMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMMIS_W<'a> {
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
#[doc = "Field `DSRMMIS` reader - "]
pub struct DSRMMIS_R(crate::FieldReader<bool, bool>);
impl DSRMMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMMIS` writer - "]
pub struct DSRMMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMMIS_W<'a> {
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
#[doc = "Field `RXMIS` reader - "]
pub struct RXMIS_R(crate::FieldReader<bool, bool>);
impl RXMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMIS` writer - "]
pub struct RXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMIS_W<'a> {
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
#[doc = "Field `TXMIS` reader - "]
pub struct TXMIS_R(crate::FieldReader<bool, bool>);
impl TXMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIS` writer - "]
pub struct TXMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMIS_W<'a> {
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
#[doc = "Field `RTMIS` reader - "]
pub struct RTMIS_R(crate::FieldReader<bool, bool>);
impl RTMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIS` writer - "]
pub struct RTMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIS_W<'a> {
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
#[doc = "Field `FEMIS` reader - "]
pub struct FEMIS_R(crate::FieldReader<bool, bool>);
impl FEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEMIS` writer - "]
pub struct FEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FEMIS_W<'a> {
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
#[doc = "Field `PEMIS` reader - "]
pub struct PEMIS_R(crate::FieldReader<bool, bool>);
impl PEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEMIS` writer - "]
pub struct PEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `BEMIS` reader - "]
pub struct BEMIS_R(crate::FieldReader<bool, bool>);
impl BEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEMIS` writer - "]
pub struct BEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `OEMIS` reader - "]
pub struct OEMIS_R(crate::FieldReader<bool, bool>);
impl OEMIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        OEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEMIS` writer - "]
pub struct OEMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OEMIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rimmis(&mut self) -> RIMMIS_W {
        RIMMIS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ctsmmis(&mut self) -> CTSMMIS_W {
        CTSMMIS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dcdmmis(&mut self) -> DCDMMIS_W {
        DCDMMIS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dsrmmis(&mut self) -> DSRMMIS_W {
        DSRMMIS_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxmis(&mut self) -> RXMIS_W {
        RXMIS_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn txmis(&mut self) -> TXMIS_W {
        TXMIS_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rtmis(&mut self) -> RTMIS_W {
        RTMIS_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn femis(&mut self) -> FEMIS_W {
        FEMIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pemis(&mut self) -> PEMIS_W {
        PEMIS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bemis(&mut self) -> BEMIS_W {
        BEMIS_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oemis(&mut self) -> OEMIS_W {
        OEMIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Masked Interrupt Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mis::W](W) writer structure"]
impl crate::Writable for MIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
