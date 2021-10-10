#[doc = "Register `ADC2_CFG` reader"]
pub struct R(crate::R<ADC2_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_CFG` writer"]
pub struct W(crate::W<ADC2_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_CFG_SPEC>;
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
impl From<crate::W<ADC2_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Cfg_REG_ADON` reader - "]
pub struct CFG_REG_ADON_R(crate::FieldReader<bool, bool>);
impl CFG_REG_ADON_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_ADON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_ADON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_ADON` writer - "]
pub struct CFG_REG_ADON_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_ADON_W<'a> {
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
#[doc = "Field `Cfg_REG_GO` reader - "]
pub struct CFG_REG_GO_R(crate::FieldReader<bool, bool>);
impl CFG_REG_GO_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_GO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_GO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_GO` writer - "]
pub struct CFG_REG_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_GO_W<'a> {
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
#[doc = "Field `Cfg_REG_CLKS` reader - "]
pub struct CFG_REG_CLKS_R(crate::FieldReader<bool, bool>);
impl CFG_REG_CLKS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_CLKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_CLKS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_CLKS` writer - "]
pub struct CFG_REG_CLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_CLKS_W<'a> {
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
#[doc = "Field `Cfg_REG_SAMPLE` reader - "]
pub struct CFG_REG_SAMPLE_R(crate::FieldReader<bool, bool>);
impl CFG_REG_SAMPLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_SAMPLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_SAMPLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_SAMPLE` writer - "]
pub struct CFG_REG_SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_SAMPLE_W<'a> {
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
#[doc = "Field `Cfg_REG_CHS` reader - "]
pub struct CFG_REG_CHS_R(crate::FieldReader<u8, u8>);
impl CFG_REG_CHS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG_REG_CHS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_CHS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_CHS` writer - "]
pub struct CFG_REG_CHS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_CHS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 4)) | ((value as u32 & 0x1f) << 4);
        self.w
    }
}
#[doc = "Field `Cfg_REG_CHCH` reader - "]
pub struct CFG_REG_CHCH_R(crate::FieldReader<bool, bool>);
impl CFG_REG_CHCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_CHCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_CHCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_CHCH` writer - "]
pub struct CFG_REG_CHCH_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_CHCH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `Cfg_REG_RNGC` reader - "]
pub struct CFG_REG_RNGC_R(crate::FieldReader<bool, bool>);
impl CFG_REG_RNGC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_REG_RNGC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_RNGC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_RNGC` writer - "]
pub struct CFG_REG_RNGC_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_RNGC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `Cfg_M_REF` reader - "]
pub struct CFG_M_REF_R(crate::FieldReader<bool, bool>);
impl CFG_M_REF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_M_REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_M_REF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_M_REF` writer - "]
pub struct CFG_M_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_M_REF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `Cfg_REG_DIVCLK` reader - "]
pub struct CFG_REG_DIVCLK_R(crate::FieldReader<u8, u8>);
impl CFG_REG_DIVCLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CFG_REG_DIVCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_REG_DIVCLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_REG_DIVCLK` writer - "]
pub struct CFG_REG_DIVCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_REG_DIVCLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `ADC1_OP` reader - "]
pub struct ADC1_OP_R(crate::FieldReader<bool, bool>);
impl ADC1_OP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC1_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC1_OP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC1_OP` writer - "]
pub struct ADC1_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_OP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ADC2_OP` reader - "]
pub struct ADC2_OP_R(crate::FieldReader<bool, bool>);
impl ADC2_OP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC2_OP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC2_OP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC2_OP` writer - "]
pub struct ADC2_OP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_OP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `Delay_Go` reader - "]
pub struct DELAY_GO_R(crate::FieldReader<u8, u8>);
impl DELAY_GO_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY_GO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_GO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay_Go` writer - "]
pub struct DELAY_GO_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_GO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | ((value as u32 & 0x07) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfg_reg_adon(&self) -> CFG_REG_ADON_R {
        CFG_REG_ADON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_reg_go(&self) -> CFG_REG_GO_R {
        CFG_REG_GO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_reg_clks(&self) -> CFG_REG_CLKS_R {
        CFG_REG_CLKS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_reg_sample(&self) -> CFG_REG_SAMPLE_R {
        CFG_REG_SAMPLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn cfg_reg_chs(&self) -> CFG_REG_CHS_R {
        CFG_REG_CHS_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_reg_chch(&self) -> CFG_REG_CHCH_R {
        CFG_REG_CHCH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_reg_rngc(&self) -> CFG_REG_RNGC_R {
        CFG_REG_RNGC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cfg_m_ref(&self) -> CFG_M_REF_R {
        CFG_M_REF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cfg_reg_divclk(&self) -> CFG_REG_DIVCLK_R {
        CFG_REG_DIVCLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adc1_op(&self) -> ADC1_OP_R {
        ADC1_OP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adc2_op(&self) -> ADC2_OP_R {
        ADC2_OP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn delay_go(&self) -> DELAY_GO_R {
        DELAY_GO_R::new(((self.bits >> 25) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfg_reg_adon(&mut self) -> CFG_REG_ADON_W {
        CFG_REG_ADON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_reg_go(&mut self) -> CFG_REG_GO_W {
        CFG_REG_GO_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_reg_clks(&mut self) -> CFG_REG_CLKS_W {
        CFG_REG_CLKS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_reg_sample(&mut self) -> CFG_REG_SAMPLE_W {
        CFG_REG_SAMPLE_W { w: self }
    }
    #[doc = "Bits 4:8"]
    #[inline(always)]
    pub fn cfg_reg_chs(&mut self) -> CFG_REG_CHS_W {
        CFG_REG_CHS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cfg_reg_chch(&mut self) -> CFG_REG_CHCH_W {
        CFG_REG_CHCH_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cfg_reg_rngc(&mut self) -> CFG_REG_RNGC_W {
        CFG_REG_RNGC_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cfg_m_ref(&mut self) -> CFG_M_REF_W {
        CFG_M_REF_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cfg_reg_divclk(&mut self) -> CFG_REG_DIVCLK_W {
        CFG_REG_DIVCLK_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn adc1_op(&mut self) -> ADC1_OP_W {
        ADC1_OP_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adc2_op(&mut self) -> ADC2_OP_W {
        ADC2_OP_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn delay_go(&mut self) -> DELAY_GO_W {
        DELAY_GO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_cfg](index.html) module"]
pub struct ADC2_CFG_SPEC;
impl crate::RegisterSpec for ADC2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_cfg::R](R) reader structure"]
impl crate::Readable for ADC2_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_cfg::W](W) writer structure"]
impl crate::Writable for ADC2_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2_CFG to value 0"]
impl crate::Resettable for ADC2_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
