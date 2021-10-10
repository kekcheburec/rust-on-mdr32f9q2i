#[doc = "Register `SSP_CLOCK` reader"]
pub struct R(crate::R<SSP_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSP_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSP_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSP_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSP_CLOCK` writer"]
pub struct W(crate::W<SSP_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSP_CLOCK_SPEC>;
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
impl From<crate::W<SSP_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSP_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSP1_BRG` reader - "]
pub struct SSP1_BRG_R(crate::FieldReader<u8, u8>);
impl SSP1_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSP1_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSP1_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP1_BRG` writer - "]
pub struct SSP1_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `SSP2_BRG` reader - "]
pub struct SSP2_BRG_R(crate::FieldReader<u8, u8>);
impl SSP2_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SSP2_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSP2_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP2_BRG` writer - "]
pub struct SSP2_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP2_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `SSP1_CLK_EN` reader - "]
pub struct SSP1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SSP1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSP1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP1_CLK_EN` writer - "]
pub struct SSP1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP1_CLK_EN_W<'a> {
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
#[doc = "Field `SSP2_CLK_EN` reader - "]
pub struct SSP2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SSP2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSP2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSP2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSP2_CLK_EN` writer - "]
pub struct SSP2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSP2_CLK_EN_W<'a> {
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
    pub fn ssp1_brg(&self) -> SSP1_BRG_R {
        SSP1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ssp2_brg(&self) -> SSP2_BRG_R {
        SSP2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ssp1_clk_en(&self) -> SSP1_CLK_EN_R {
        SSP1_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ssp2_clk_en(&self) -> SSP2_CLK_EN_R {
        SSP2_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ssp1_brg(&mut self) -> SSP1_BRG_W {
        SSP1_BRG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ssp2_brg(&mut self) -> SSP2_BRG_W {
        SSP2_BRG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ssp1_clk_en(&mut self) -> SSP1_CLK_EN_W {
        SSP1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ssp2_clk_en(&mut self) -> SSP2_CLK_EN_W {
        SSP2_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSP Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssp_clock](index.html) module"]
pub struct SSP_CLOCK_SPEC;
impl crate::RegisterSpec for SSP_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssp_clock::R](R) reader structure"]
impl crate::Readable for SSP_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssp_clock::W](W) writer structure"]
impl crate::Writable for SSP_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSP_CLOCK to value 0"]
impl crate::Resettable for SSP_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
