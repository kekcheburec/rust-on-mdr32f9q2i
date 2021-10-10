#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER_ENABLE` reader - "]
pub struct MASTER_ENABLE_R(crate::FieldReader<bool, bool>);
impl MASTER_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASTER_ENABLE` writer - "]
pub struct MASTER_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASTER_ENABLE_W<'a> {
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
#[doc = "Field `STATE` reader - "]
pub struct STATE_R(crate::FieldReader<u8, u8>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATE` writer - "]
pub struct STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `CHNLS_MINUS1` reader - "]
pub struct CHNLS_MINUS1_R(crate::FieldReader<u8, u8>);
impl CHNLS_MINUS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHNLS_MINUS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHNLS_MINUS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHNLS_MINUS1` writer - "]
pub struct CHNLS_MINUS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNLS_MINUS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `TEST_STATUS` reader - "]
pub struct TEST_STATUS_R(crate::FieldReader<u8, u8>);
impl TEST_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_STATUS` writer - "]
pub struct TEST_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn master_enable(&self) -> MASTER_ENABLE_R {
        MASTER_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn chnls_minus1(&self) -> CHNLS_MINUS1_R {
        CHNLS_MINUS1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_status(&self) -> TEST_STATUS_R {
        TEST_STATUS_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn master_enable(&mut self) -> MASTER_ENABLE_W {
        MASTER_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn state(&mut self) -> STATE_W {
        STATE_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn chnls_minus1(&mut self) -> CHNLS_MINUS1_W {
        CHNLS_MINUS1_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_status(&mut self) -> TEST_STATUS_W {
        TEST_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
