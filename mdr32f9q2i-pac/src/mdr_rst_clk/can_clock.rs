#[doc = "Register `CAN_CLOCK` reader"]
pub struct R(crate::R<CAN_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAN_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAN_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAN_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAN_CLOCK` writer"]
pub struct W(crate::W<CAN_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAN_CLOCK_SPEC>;
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
impl From<crate::W<CAN_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAN_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN1_BRG` reader - "]
pub struct CAN1_BRG_R(crate::FieldReader<u8, u8>);
impl CAN1_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAN1_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1_BRG` writer - "]
pub struct CAN1_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CAN2_BRG` reader - "]
pub struct CAN2_BRG_R(crate::FieldReader<u8, u8>);
impl CAN2_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAN2_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN2_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN2_BRG` writer - "]
pub struct CAN2_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CAN1_CLK_EN` reader - "]
pub struct CAN1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CAN1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1_CLK_EN` writer - "]
pub struct CAN1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CAN2_CLK_EN` reader - "]
pub struct CAN2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl CAN2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN2_CLK_EN` writer - "]
pub struct CAN2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn can1_brg(&self) -> CAN1_BRG_R {
        CAN1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn can2_brg(&self) -> CAN2_BRG_R {
        CAN2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn can1_clk_en(&self) -> CAN1_CLK_EN_R {
        CAN1_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn can2_clk_en(&self) -> CAN2_CLK_EN_R {
        CAN2_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn can1_brg(&mut self) -> CAN1_BRG_W {
        CAN1_BRG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn can2_brg(&mut self) -> CAN2_BRG_W {
        CAN2_BRG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn can1_clk_en(&mut self) -> CAN1_CLK_EN_W {
        CAN1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn can2_clk_en(&mut self) -> CAN2_CLK_EN_W {
        CAN2_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [can_clock](index.html) module"]
pub struct CAN_CLOCK_SPEC;
impl crate::RegisterSpec for CAN_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [can_clock::R](R) reader structure"]
impl crate::Readable for CAN_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [can_clock::W](W) writer structure"]
impl crate::Writable for CAN_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAN_CLOCK to value 0"]
impl crate::Resettable for CAN_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
