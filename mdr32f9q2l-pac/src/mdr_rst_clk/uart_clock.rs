#[doc = "Register `UART_CLOCK` reader"]
pub struct R(crate::R<UART_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_CLOCK` writer"]
pub struct W(crate::W<UART_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_CLOCK_SPEC>;
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
impl From<crate::W<UART_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART1_BRG` reader - "]
pub struct UART1_BRG_R(crate::FieldReader<u8, u8>);
impl UART1_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART1_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_BRG` writer - "]
pub struct UART1_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `UART2_BRG` reader - "]
pub struct UART2_BRG_R(crate::FieldReader<u8, u8>);
impl UART2_BRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART2_BRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_BRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_BRG` writer - "]
pub struct UART2_BRG_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_BRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `UART1_CLK_EN` reader - "]
pub struct UART1_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART1_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART1_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1_CLK_EN` writer - "]
pub struct UART1_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_EN_W<'a> {
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
#[doc = "Field `UART2_CLK_EN` reader - "]
pub struct UART2_CLK_EN_R(crate::FieldReader<bool, bool>);
impl UART2_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART2_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART2_CLK_EN` writer - "]
pub struct UART2_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CLK_EN_W<'a> {
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
    pub fn uart1_brg(&self) -> UART1_BRG_R {
        UART1_BRG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn uart2_brg(&self) -> UART2_BRG_R {
        UART2_BRG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart1_clk_en(&self) -> UART1_CLK_EN_R {
        UART1_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart1_brg(&mut self) -> UART1_BRG_W {
        UART1_BRG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn uart2_brg(&mut self) -> UART2_BRG_W {
        UART2_BRG_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn uart1_clk_en(&mut self) -> UART1_CLK_EN_W {
        UART1_CLK_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W {
        UART2_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_clock](index.html) module"]
pub struct UART_CLOCK_SPEC;
impl crate::RegisterSpec for UART_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_clock::R](R) reader structure"]
impl crate::Readable for UART_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_clock::W](W) writer structure"]
impl crate::Writable for UART_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_CLOCK to value 0"]
impl crate::Resettable for UART_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
