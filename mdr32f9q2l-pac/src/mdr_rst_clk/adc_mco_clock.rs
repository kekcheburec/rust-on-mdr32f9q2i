#[doc = "Register `ADC_MCO_CLOCK` reader"]
pub struct R(crate::R<ADC_MCO_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_MCO_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_MCO_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_MCO_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_MCO_CLOCK` writer"]
pub struct W(crate::W<ADC_MCO_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_MCO_CLOCK_SPEC>;
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
impl From<crate::W<ADC_MCO_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_MCO_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC_C1_SEL` reader - "]
pub struct ADC_C1_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_C1_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_C1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_C1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_C1_SEL` writer - "]
pub struct ADC_C1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ADC_C2_SEL` reader - "]
pub struct ADC_C2_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_C2_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_C2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_C2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_C2_SEL` writer - "]
pub struct ADC_C2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ADC_C3_SEL` reader - "]
pub struct ADC_C3_SEL_R(crate::FieldReader<u8, u8>);
impl ADC_C3_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC_C3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_C3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_C3_SEL` writer - "]
pub struct ADC_C3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_C3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MCO_EN` reader - "]
pub struct MCO_EN_R(crate::FieldReader<bool, bool>);
impl MCO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO_EN` writer - "]
pub struct MCO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO_EN_W<'a> {
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
#[doc = "Field `ADC_CLK_EN` reader - "]
pub struct ADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC_CLK_EN` writer - "]
pub struct ADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_c1_sel(&self) -> ADC_C1_SEL_R {
        ADC_C1_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_c2_sel(&self) -> ADC_C2_SEL_R {
        ADC_C2_SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc_c3_sel(&self) -> ADC_C3_SEL_R {
        ADC_C3_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mco_en(&self) -> MCO_EN_R {
        MCO_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adc_c1_sel(&mut self) -> ADC_C1_SEL_W {
        ADC_C1_SEL_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn adc_c2_sel(&mut self) -> ADC_C2_SEL_W {
        ADC_C2_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn adc_c3_sel(&mut self) -> ADC_C3_SEL_W {
        ADC_C3_SEL_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn mco_en(&mut self) -> MCO_EN_W {
        MCO_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W {
        ADC_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_mco_clock](index.html) module"]
pub struct ADC_MCO_CLOCK_SPEC;
impl crate::RegisterSpec for ADC_MCO_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_mco_clock::R](R) reader structure"]
impl crate::Readable for ADC_MCO_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_mco_clock::W](W) writer structure"]
impl crate::Writable for ADC_MCO_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_MCO_CLOCK to value 0"]
impl crate::Resettable for ADC_MCO_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
