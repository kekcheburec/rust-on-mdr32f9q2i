#[doc = "Register `TIM_CLOCK` reader"]
pub struct R(crate::R<TIM_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM_CLOCK` writer"]
pub struct W(crate::W<TIM_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_CLOCK_SPEC>;
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
impl From<crate::W<TIM_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1_BRG` reader - "]
pub struct TIM1_BRG_R(crate::FieldReader<u8, u8>);
impl TIM1_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM1_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_BRG` writer - "]
pub struct TIM1_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `TIM2_BRG` reader - "]
pub struct TIM2_BRG_R(crate::FieldReader<u8, u8>);
impl TIM2_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM2_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2_BRG` writer - "]
pub struct TIM2_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `TIM3_BRG` reader - "]
pub struct TIM3_BRG_R(crate::FieldReader<u8, u8>);
impl TIM3_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIM3_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3_BRG` writer - "]
pub struct TIM3_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TIM1_CLK_EN` reader - "]
pub struct TIM1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIM1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM1_CLK_EN` writer - "]
pub struct TIM1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM1_CLK_EN_W<'a> {
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
#[doc = "Field `TIM2_CLK_EN` reader - "]
pub struct TIM2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIM2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM2_CLK_EN` writer - "]
pub struct TIM2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2_CLK_EN_W<'a> {
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
#[doc = "Field `TIM3_CLK_EN` reader - "]
pub struct TIM3_CLK_EN_R(crate::FieldReader<bool, bool>);
impl TIM3_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIM3_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIM3_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIM3_CLK_EN` writer - "]
pub struct TIM3_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM3_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tim1_brg(&self) -> TIM1_BRG_R {
        TIM1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tim2_brg(&self) -> TIM2_BRG_R {
        TIM2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tim3_brg(&self) -> TIM3_BRG_R {
        TIM3_BRG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim1_clk_en(&self) -> TIM1_CLK_EN_R {
        TIM1_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim2_clk_en(&self) -> TIM2_CLK_EN_R {
        TIM2_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tim3_clk_en(&self) -> TIM3_CLK_EN_R {
        TIM3_CLK_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tim1_brg(&mut self) -> TIM1_BRG_W {
        TIM1_BRG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tim2_brg(&mut self) -> TIM2_BRG_W {
        TIM2_BRG_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tim3_brg(&mut self) -> TIM3_BRG_W {
        TIM3_BRG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tim1_clk_en(&mut self) -> TIM1_CLK_EN_W {
        TIM1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tim2_clk_en(&mut self) -> TIM2_CLK_EN_W {
        TIM2_CLK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tim3_clk_en(&mut self) -> TIM3_CLK_EN_W {
        TIM3_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim_clock](index.html) module"]
pub struct TIM_CLOCK_SPEC;
impl crate::RegisterSpec for TIM_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim_clock::R](R) reader structure"]
impl crate::Readable for TIM_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim_clock::W](W) writer structure"]
impl crate::Writable for TIM_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM_CLOCK to value 0"]
impl crate::Resettable for TIM_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
