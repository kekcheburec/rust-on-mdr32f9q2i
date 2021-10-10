#[doc = "Register `BITTMNG` reader"]
pub struct R(crate::R<BITTMNG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BITTMNG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BITTMNG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BITTMNG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BITTMNG` writer"]
pub struct W(crate::W<BITTMNG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BITTMNG_SPEC>;
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
impl From<crate::W<BITTMNG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BITTMNG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - "]
pub struct BRP_R(crate::FieldReader<u16, u16>);
impl BRP_R {
    pub(crate) fn new(bits: u16) -> Self {
        BRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRP` writer - "]
pub struct BRP_W<'a> {
    w: &'a mut W,
}
impl<'a> BRP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
#[doc = "Field `PSEG` reader - "]
pub struct PSEG_R(crate::FieldReader<u8, u8>);
impl PSEG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEG` writer - "]
pub struct PSEG_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `SEG1` reader - "]
pub struct SEG1_R(crate::FieldReader<u8, u8>);
impl SEG1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEG1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEG1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEG1` writer - "]
pub struct SEG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `SEG2` reader - "]
pub struct SEG2_R(crate::FieldReader<u8, u8>);
impl SEG2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SEG2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEG2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEG2` writer - "]
pub struct SEG2_W<'a> {
    w: &'a mut W,
}
impl<'a> SEG2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `SJW` reader - "]
pub struct SJW_R(crate::FieldReader<u8, u8>);
impl SJW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SJW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SJW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SJW` writer - "]
pub struct SJW_W<'a> {
    w: &'a mut W,
}
impl<'a> SJW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | ((value as u32 & 0x03) << 25);
        self.w
    }
}
#[doc = "Field `SB` reader - "]
pub struct SB_R(crate::FieldReader<bool, bool>);
impl SB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB` writer - "]
pub struct SB_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pseg(&self) -> PSEG_R {
        PSEG_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn seg1(&self) -> SEG1_R {
        SEG1_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn seg2(&self) -> SEG2_R {
        SEG2_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn brp(&mut self) -> BRP_W {
        BRP_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn pseg(&mut self) -> PSEG_W {
        PSEG_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn seg1(&mut self) -> SEG1_W {
        SEG1_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn seg2(&mut self) -> SEG2_W {
        SEG2_W { w: self }
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SJW_W {
        SJW_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sb(&mut self) -> SB_W {
        SB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Bittiming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bittmng](index.html) module"]
pub struct BITTMNG_SPEC;
impl crate::RegisterSpec for BITTMNG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bittmng::R](R) reader structure"]
impl crate::Readable for BITTMNG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bittmng::W](W) writer structure"]
impl crate::Writable for BITTMNG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BITTMNG to value 0"]
impl crate::Resettable for BITTMNG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
