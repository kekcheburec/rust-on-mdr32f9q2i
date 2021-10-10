#[doc = "Register `CPU_CLOCK` reader"]
pub struct R(crate::R<CPU_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPU_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPU_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPU_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPU_CLOCK` writer"]
pub struct W(crate::W<CPU_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPU_CLOCK_SPEC>;
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
impl From<crate::W<CPU_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPU_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_C1_SEL` reader - "]
pub struct CPU_C1_SEL_R(crate::FieldReader<u8, u8>);
impl CPU_C1_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_C1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_C1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_C1_SEL` writer - "]
pub struct CPU_C1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_C1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CPU_C2_SEL` reader - "]
pub struct CPU_C2_SEL_R(crate::FieldReader<bool, bool>);
impl CPU_C2_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_C2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_C2_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_C2_SEL` writer - "]
pub struct CPU_C2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_C2_SEL_W<'a> {
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
#[doc = "Field `CPU_C3_SEL` reader - "]
pub struct CPU_C3_SEL_R(crate::FieldReader<u8, u8>);
impl CPU_C3_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPU_C3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_C3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_C3_SEL` writer - "]
pub struct CPU_C3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_C3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `HCLK_SEL` reader - "]
pub struct HCLK_SEL_R(crate::FieldReader<u8, u8>);
impl HCLK_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HCLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCLK_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCLK_SEL` writer - "]
pub struct HCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpu_c1_sel(&self) -> CPU_C1_SEL_R {
        CPU_C1_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpu_c2_sel(&self) -> CPU_C2_SEL_R {
        CPU_C2_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cpu_c3_sel(&self) -> CPU_C3_SEL_R {
        CPU_C3_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn hclk_sel(&self) -> HCLK_SEL_R {
        HCLK_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cpu_c1_sel(&mut self) -> CPU_C1_SEL_W {
        CPU_C1_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cpu_c2_sel(&mut self) -> CPU_C2_SEL_W {
        CPU_C2_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cpu_c3_sel(&mut self) -> CPU_C3_SEL_W {
        CPU_C3_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn hclk_sel(&mut self) -> HCLK_SEL_W {
        HCLK_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpu_clock](index.html) module"]
pub struct CPU_CLOCK_SPEC;
impl crate::RegisterSpec for CPU_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpu_clock::R](R) reader structure"]
impl crate::Readable for CPU_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpu_clock::W](W) writer structure"]
impl crate::Writable for CPU_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CPU_CLOCK to value 0"]
impl crate::Resettable for CPU_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
