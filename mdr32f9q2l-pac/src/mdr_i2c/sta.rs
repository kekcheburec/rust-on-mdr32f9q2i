#[doc = "Register `STA` reader"]
pub struct R(crate::R<STA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STA` writer"]
pub struct W(crate::W<STA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STA_SPEC>;
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
impl From<crate::W<STA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - "]
pub struct INT_R(crate::FieldReader<bool, bool>);
impl INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - "]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
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
#[doc = "Field `TR_PROG` reader - "]
pub struct TR_PROG_R(crate::FieldReader<bool, bool>);
impl TR_PROG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TR_PROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_PROG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR_PROG` writer - "]
pub struct TR_PROG_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_PROG_W<'a> {
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
#[doc = "Field `LOST_ARB` reader - "]
pub struct LOST_ARB_R(crate::FieldReader<bool, bool>);
impl LOST_ARB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOST_ARB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOST_ARB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOST_ARB` writer - "]
pub struct LOST_ARB_W<'a> {
    w: &'a mut W,
}
impl<'a> LOST_ARB_W<'a> {
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
#[doc = "Field `BUSY` reader - "]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - "]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `Rx_ACK` reader - "]
pub struct RX_ACK_R(crate::FieldReader<bool, bool>);
impl RX_ACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Rx_ACK` writer - "]
pub struct RX_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ACK_W<'a> {
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
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tr_prog(&self) -> TR_PROG_R {
        TR_PROG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lost_arb(&self) -> LOST_ARB_R {
        LOST_ARB_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_ack(&self) -> RX_ACK_R {
        RX_ACK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tr_prog(&mut self) -> TR_PROG_W {
        TR_PROG_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn lost_arb(&mut self) -> LOST_ARB_W {
        LOST_ARB_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_ack(&mut self) -> RX_ACK_W {
        RX_ACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sta](index.html) module"]
pub struct STA_SPEC;
impl crate::RegisterSpec for STA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sta::R](R) reader structure"]
impl crate::Readable for STA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sta::W](W) writer structure"]
impl crate::Writable for STA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STA to value 0"]
impl crate::Resettable for STA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
