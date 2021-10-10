#[doc = "Register `REG_0F` reader"]
pub struct R(crate::R<REG_0F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_0F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_0F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_0F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_0F` writer"]
pub struct W(crate::W<REG_0F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_0F_SPEC>;
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
impl From<crate::W<REG_0F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_0F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSE_ON` reader - "]
pub struct LSE_ON_R(crate::FieldReader<bool, bool>);
impl LSE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE_ON` writer - "]
pub struct LSE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_ON_W<'a> {
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
#[doc = "Field `LSE_BYP` reader - "]
pub struct LSE_BYP_R(crate::FieldReader<bool, bool>);
impl LSE_BYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSE_BYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_BYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE_BYP` writer - "]
pub struct LSE_BYP_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_BYP_W<'a> {
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
#[doc = "Field `RTC_SEL` reader - "]
pub struct RTC_SEL_R(crate::FieldReader<u8, u8>);
impl RTC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_SEL` writer - "]
pub struct RTC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `RTC_EN` reader - "]
pub struct RTC_EN_R(crate::FieldReader<bool, bool>);
impl RTC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_EN` writer - "]
pub struct RTC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CAL` reader - "]
pub struct CAL_R(crate::FieldReader<u8, u8>);
impl CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAL` writer - "]
pub struct CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 5)) | ((value as u32 & 0xff) << 5);
        self.w
    }
}
#[doc = "Field `LSE_RDY` reader - "]
pub struct LSE_RDY_R(crate::FieldReader<bool, bool>);
impl LSE_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSE_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSE_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSE_RDY` writer - "]
pub struct LSE_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LSE_RDY_W<'a> {
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
#[doc = "Field `BKP_REG` reader - "]
pub struct BKP_REG_R(crate::FieldReader<bool, bool>);
impl BKP_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        BKP_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP_REG` writer - "]
pub struct BKP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_REG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `LSI_ON` reader - "]
pub struct LSI_ON_R(crate::FieldReader<bool, bool>);
impl LSI_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI_ON` writer - "]
pub struct LSI_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `LSI_TRIM` reader - "]
pub struct LSI_TRIM_R(crate::FieldReader<u8, u8>);
impl LSI_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        LSI_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI_TRIM` writer - "]
pub struct LSI_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `LSI_RDY` reader - "]
pub struct LSI_RDY_R(crate::FieldReader<bool, bool>);
impl LSI_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSI_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LSI_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSI_RDY` writer - "]
pub struct LSI_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LSI_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `HSI_ON` reader - "]
pub struct HSI_ON_R(crate::FieldReader<bool, bool>);
impl HSI_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI_ON` writer - "]
pub struct HSI_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `HSI_RDY` reader - "]
pub struct HSI_RDY_R(crate::FieldReader<bool, bool>);
impl HSI_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSI_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI_RDY` writer - "]
pub struct HSI_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `HSI_TRIM` reader - "]
pub struct HSI_TRIM_R(crate::FieldReader<u8, u8>);
impl HSI_TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSI_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSI_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSI_TRIM` writer - "]
pub struct HSI_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSI_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Field `STANDBY` reader - "]
pub struct STANDBY_R(crate::FieldReader<bool, bool>);
impl STANDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STANDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STANDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STANDBY` writer - "]
pub struct STANDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> STANDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `RTC_RESET` reader - "]
pub struct RTC_RESET_R(crate::FieldReader<bool, bool>);
impl RTC_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_RESET` writer - "]
pub struct RTC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lse_on(&self) -> LSE_ON_R {
        LSE_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lse_byp(&self) -> LSE_BYP_R {
        LSE_BYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rtc_sel(&self) -> RTC_SEL_R {
        RTC_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_en(&self) -> RTC_EN_R {
        RTC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lse_rdy(&self) -> LSE_RDY_R {
        LSE_RDY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lsi_on(&self) -> LSI_ON_R {
        LSI_ON_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lsi_trim(&self) -> LSI_TRIM_R {
        LSI_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lsi_rdy(&self) -> LSI_RDY_R {
        LSI_RDY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hsi_on(&self) -> HSI_ON_R {
        HSI_ON_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn hsi_rdy(&self) -> HSI_RDY_R {
        HSI_RDY_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn hsi_trim(&self) -> HSI_TRIM_R {
        HSI_TRIM_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn standby(&self) -> STANDBY_R {
        STANDBY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RTC_RESET_R {
        RTC_RESET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lse_on(&mut self) -> LSE_ON_W {
        LSE_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lse_byp(&mut self) -> LSE_BYP_W {
        LSE_BYP_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rtc_sel(&mut self) -> RTC_SEL_W {
        RTC_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rtc_en(&mut self) -> RTC_EN_W {
        RTC_EN_W { w: self }
    }
    #[doc = "Bits 5:12"]
    #[inline(always)]
    pub fn cal(&mut self) -> CAL_W {
        CAL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn lse_rdy(&mut self) -> LSE_RDY_W {
        LSE_RDY_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn bkp_reg(&mut self) -> BKP_REG_W {
        BKP_REG_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn lsi_on(&mut self) -> LSI_ON_W {
        LSI_ON_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn lsi_trim(&mut self) -> LSI_TRIM_W {
        LSI_TRIM_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn lsi_rdy(&mut self) -> LSI_RDY_W {
        LSI_RDY_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn hsi_on(&mut self) -> HSI_ON_W {
        HSI_ON_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn hsi_rdy(&mut self) -> HSI_RDY_W {
        HSI_RDY_W { w: self }
    }
    #[doc = "Bits 24:29"]
    #[inline(always)]
    pub fn hsi_trim(&mut self) -> HSI_TRIM_W {
        HSI_TRIM_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn standby(&mut self) -> STANDBY_W {
        STANDBY_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_reset(&mut self) -> RTC_RESET_W {
        RTC_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_0f](index.html) module"]
pub struct REG_0F_SPEC;
impl crate::RegisterSpec for REG_0F_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_0f::R](R) reader structure"]
impl crate::Readable for REG_0F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_0f::W](W) writer structure"]
impl crate::Writable for REG_0F_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_0F to value 0"]
impl crate::Resettable for REG_0F_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
