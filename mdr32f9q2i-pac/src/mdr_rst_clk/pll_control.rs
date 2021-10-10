#[doc = "Register `PLL_CONTROL` reader"]
pub struct R(crate::R<PLL_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_CONTROL` writer"]
pub struct W(crate::W<PLL_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_CONTROL_SPEC>;
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
impl From<crate::W<PLL_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_USB_ON` reader - "]
pub struct PLL_USB_ON_R(crate::FieldReader<bool, bool>);
impl PLL_USB_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_USB_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_USB_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_USB_ON` writer - "]
pub struct PLL_USB_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_USB_ON_W<'a> {
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
#[doc = "Field `PLL_USB_RLD` reader - "]
pub struct PLL_USB_RLD_R(crate::FieldReader<bool, bool>);
impl PLL_USB_RLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_USB_RLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_USB_RLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_USB_RLD` writer - "]
pub struct PLL_USB_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_USB_RLD_W<'a> {
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
#[doc = "Field `PLL_CPU_ON` reader - "]
pub struct PLL_CPU_ON_R(crate::FieldReader<bool, bool>);
impl PLL_CPU_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_CPU_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CPU_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CPU_ON` writer - "]
pub struct PLL_CPU_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CPU_ON_W<'a> {
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
#[doc = "Field `PLL_CPU_PLD` reader - "]
pub struct PLL_CPU_PLD_R(crate::FieldReader<bool, bool>);
impl PLL_CPU_PLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL_CPU_PLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CPU_PLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CPU_PLD` writer - "]
pub struct PLL_CPU_PLD_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CPU_PLD_W<'a> {
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
#[doc = "Field `PLL_USB_MUL` reader - "]
pub struct PLL_USB_MUL_R(crate::FieldReader<u8, u8>);
impl PLL_USB_MUL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL_USB_MUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_USB_MUL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_USB_MUL` writer - "]
pub struct PLL_USB_MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_USB_MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `PLL_CPU_MUL` reader - "]
pub struct PLL_CPU_MUL_R(crate::FieldReader<u8, u8>);
impl PLL_CPU_MUL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL_CPU_MUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL_CPU_MUL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL_CPU_MUL` writer - "]
pub struct PLL_CPU_MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_CPU_MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pll_usb_on(&self) -> PLL_USB_ON_R {
        PLL_USB_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pll_usb_rld(&self) -> PLL_USB_RLD_R {
        PLL_USB_RLD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_cpu_on(&self) -> PLL_CPU_ON_R {
        PLL_CPU_ON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pll_cpu_pld(&self) -> PLL_CPU_PLD_R {
        PLL_CPU_PLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pll_usb_mul(&self) -> PLL_USB_MUL_R {
        PLL_USB_MUL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pll_cpu_mul(&self) -> PLL_CPU_MUL_R {
        PLL_CPU_MUL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pll_usb_on(&mut self) -> PLL_USB_ON_W {
        PLL_USB_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pll_usb_rld(&mut self) -> PLL_USB_RLD_W {
        PLL_USB_RLD_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll_cpu_on(&mut self) -> PLL_CPU_ON_W {
        PLL_CPU_ON_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pll_cpu_pld(&mut self) -> PLL_CPU_PLD_W {
        PLL_CPU_PLD_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pll_usb_mul(&mut self) -> PLL_USB_MUL_W {
        PLL_USB_MUL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn pll_cpu_mul(&mut self) -> PLL_CPU_MUL_W {
        PLL_CPU_MUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_control](index.html) module"]
pub struct PLL_CONTROL_SPEC;
impl crate::RegisterSpec for PLL_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_control::R](R) reader structure"]
impl crate::Readable for PLL_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_control::W](W) writer structure"]
impl crate::Writable for PLL_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLL_CONTROL to value 0"]
impl crate::Resettable for PLL_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
