#[doc = "Register `CH4_CNTRL1` reader"]
pub struct R(crate::R<CH4_CNTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH4_CNTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH4_CNTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH4_CNTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH4_CNTRL1` writer"]
pub struct W(crate::W<CH4_CNTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH4_CNTRL1_SPEC>;
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
impl From<crate::W<CH4_CNTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH4_CNTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELOE` reader - "]
pub struct SELOE_R(crate::FieldReader<u8, u8>);
impl SELOE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELOE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELOE` writer - "]
pub struct SELOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SELOE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `SELO` reader - "]
pub struct SELO_R(crate::FieldReader<u8, u8>);
impl SELO_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SELO` writer - "]
pub struct SELO_W<'a> {
    w: &'a mut W,
}
impl<'a> SELO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `INV` reader - "]
pub struct INV_R(crate::FieldReader<bool, bool>);
impl INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - "]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
#[doc = "Field `NSELOE` reader - "]
pub struct NSELOE_R(crate::FieldReader<u8, u8>);
impl NSELOE_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSELOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSELOE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSELOE` writer - "]
pub struct NSELOE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSELOE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `NSELO` reader - "]
pub struct NSELO_R(crate::FieldReader<u8, u8>);
impl NSELO_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSELO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSELO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSELO` writer - "]
pub struct NSELO_W<'a> {
    w: &'a mut W,
}
impl<'a> NSELO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `NINV` reader - "]
pub struct NINV_R(crate::FieldReader<bool, bool>);
impl NINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        NINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NINV` writer - "]
pub struct NINV_W<'a> {
    w: &'a mut W,
}
impl<'a> NINV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn seloe(&self) -> SELOE_R {
        SELOE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn selo(&self) -> SELO_R {
        SELO_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn nseloe(&self) -> NSELOE_R {
        NSELOE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn nselo(&self) -> NSELO_R {
        NSELO_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ninv(&self) -> NINV_R {
        NINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn seloe(&mut self) -> SELOE_W {
        SELOE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn selo(&mut self) -> SELO_W {
        SELO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn nseloe(&mut self) -> NSELOE_W {
        NSELOE_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn nselo(&mut self) -> NSELO_W {
        NSELO_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ninv(&mut self) -> NINV_W {
        NINV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_cntrl1](index.html) module"]
pub struct CH4_CNTRL1_SPEC;
impl crate::RegisterSpec for CH4_CNTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch4_cntrl1::R](R) reader structure"]
impl crate::Readable for CH4_CNTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch4_cntrl1::W](W) writer structure"]
impl crate::Writable for CH4_CNTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH4_CNTRL1 to value 0"]
impl crate::Resettable for CH4_CNTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
