#[doc = "Register `ADC1_CFG` reader"]
pub struct R(crate::R<ADC1_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1_CFG` writer"]
pub struct W(crate::W<ADC1_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_CFG_SPEC>;
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
impl From<crate::W<ADC1_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_CFG_SPEC>) -> Self {
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
#[doc = "Field `Cfg_Sync_Conver` reader - "]
pub struct CFG_SYNC_CONVER_R(crate::FieldReader<bool, bool>);
impl CFG_SYNC_CONVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_SYNC_CONVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_SYNC_CONVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_Sync_Conver` writer - "]
pub struct CFG_SYNC_CONVER_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_SYNC_CONVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `TS_EN` reader - "]
pub struct TS_EN_R(crate::FieldReader<bool, bool>);
impl TS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_EN` writer - "]
pub struct TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_EN_W<'a> {
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
#[doc = "Field `TS_BUF_EN` reader - "]
pub struct TS_BUF_EN_R(crate::FieldReader<bool, bool>);
impl TS_BUF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TS_BUF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TS_BUF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TS_BUF_EN` writer - "]
pub struct TS_BUF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS_BUF_EN_W<'a> {
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
#[doc = "Field `SEL_TS` reader - "]
pub struct SEL_TS_R(crate::FieldReader<bool, bool>);
impl SEL_TS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_TS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_TS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_TS` writer - "]
pub struct SEL_TS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_TS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SEL_VREF` reader - "]
pub struct SEL_VREF_R(crate::FieldReader<bool, bool>);
impl SEL_VREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_VREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_VREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEL_VREF` writer - "]
pub struct SEL_VREF_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_VREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `TR` reader - "]
pub struct TR_R(crate::FieldReader<u8, u8>);
impl TR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TR` writer - "]
pub struct TR_W<'a> {
    w: &'a mut W,
}
impl<'a> TR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | ((value as u32 & 0x0f) << 21);
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
#[doc = "Field `Delay_ADC` reader - "]
pub struct DELAY_ADC_R(crate::FieldReader<u8, u8>);
impl DELAY_ADC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_ADC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay_ADC` writer - "]
pub struct DELAY_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_ADC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cfg_sync_conver(&self) -> CFG_SYNC_CONVER_R {
        CFG_SYNC_CONVER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ts_en(&self) -> TS_EN_R {
        TS_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ts_buf_en(&self) -> TS_BUF_EN_R {
        TS_BUF_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sel_ts(&self) -> SEL_TS_R {
        SEL_TS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sel_vref(&self) -> SEL_VREF_R {
        SEL_VREF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn tr(&self) -> TR_R {
        TR_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn delay_go(&self) -> DELAY_GO_R {
        DELAY_GO_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn delay_adc(&self) -> DELAY_ADC_R {
        DELAY_ADC_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cfg_sync_conver(&mut self) -> CFG_SYNC_CONVER_W {
        CFG_SYNC_CONVER_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ts_en(&mut self) -> TS_EN_W {
        TS_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ts_buf_en(&mut self) -> TS_BUF_EN_W {
        TS_BUF_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sel_ts(&mut self) -> SEL_TS_W {
        SEL_TS_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sel_vref(&mut self) -> SEL_VREF_W {
        SEL_VREF_W { w: self }
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn tr(&mut self) -> TR_W {
        TR_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn delay_go(&mut self) -> DELAY_GO_W {
        DELAY_GO_W { w: self }
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn delay_adc(&mut self) -> DELAY_ADC_W {
        DELAY_ADC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1_cfg](index.html) module"]
pub struct ADC1_CFG_SPEC;
impl crate::RegisterSpec for ADC1_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1_cfg::R](R) reader structure"]
impl crate::Readable for ADC1_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1_cfg::W](W) writer structure"]
impl crate::Writable for ADC1_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC1_CFG to value 0"]
impl crate::Resettable for ADC1_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
