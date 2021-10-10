#[doc = "Register `CH2_CNTRL` reader"]
pub struct R(crate::R<CH2_CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_CNTRL` writer"]
pub struct W(crate::W<CH2_CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_CNTRL_SPEC>;
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
impl From<crate::W<CH2_CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHFLTR` reader - "]
pub struct CHFLTR_R(crate::FieldReader<u8, u8>);
impl CHFLTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHFLTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHFLTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHFLTR` writer - "]
pub struct CHFLTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CHFLTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `CHSEL` reader - "]
pub struct CHSEL_R(crate::FieldReader<u8, u8>);
impl CHSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL` writer - "]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CHPSC` reader - "]
pub struct CHPSC_R(crate::FieldReader<u8, u8>);
impl CHPSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHPSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHPSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHPSC` writer - "]
pub struct CHPSC_W<'a> {
    w: &'a mut W,
}
impl<'a> CHPSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `OCCE` reader - "]
pub struct OCCE_R(crate::FieldReader<bool, bool>);
impl OCCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCCE` writer - "]
pub struct OCCE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `OCCM` reader - "]
pub struct OCCM_R(crate::FieldReader<u8, u8>);
impl OCCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        OCCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OCCM` writer - "]
pub struct OCCM_W<'a> {
    w: &'a mut W,
}
impl<'a> OCCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `BRKEN` reader - "]
pub struct BRKEN_R(crate::FieldReader<bool, bool>);
impl BRKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRKEN` writer - "]
pub struct BRKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRKEN_W<'a> {
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
#[doc = "Field `ETREN` reader - "]
pub struct ETREN_R(crate::FieldReader<bool, bool>);
impl ETREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETREN` writer - "]
pub struct ETREN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETREN_W<'a> {
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
#[doc = "Field `WR_CMPL` reader - "]
pub struct WR_CMPL_R(crate::FieldReader<bool, bool>);
impl WR_CMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WR_CMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WR_CMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WR_CMPL` writer - "]
pub struct WR_CMPL_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_CMPL_W<'a> {
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
#[doc = "Field `CAP_nPWM` reader - "]
pub struct CAP_NPWM_R(crate::FieldReader<bool, bool>);
impl CAP_NPWM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAP_NPWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAP_NPWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAP_nPWM` writer - "]
pub struct CAP_NPWM_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP_NPWM_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chfltr(&self) -> CHFLTR_R {
        CHFLTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chpsc(&self) -> CHPSC_R {
        CHPSC_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn occe(&self) -> OCCE_R {
        OCCE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn occm(&self) -> OCCM_R {
        OCCM_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn brken(&self) -> BRKEN_R {
        BRKEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn etren(&self) -> ETREN_R {
        ETREN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_cmpl(&self) -> WR_CMPL_R {
        WR_CMPL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cap_n_pwm(&self) -> CAP_NPWM_R {
        CAP_NPWM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn chfltr(&mut self) -> CHFLTR_W {
        CHFLTR_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn chpsc(&mut self) -> CHPSC_W {
        CHPSC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn occe(&mut self) -> OCCE_W {
        OCCE_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn occm(&mut self) -> OCCM_W {
        OCCM_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn brken(&mut self) -> BRKEN_W {
        BRKEN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn etren(&mut self) -> ETREN_W {
        ETREN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wr_cmpl(&mut self) -> WR_CMPL_W {
        WR_CMPL_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cap_n_pwm(&mut self) -> CAP_NPWM_W {
        CAP_NPWM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cntrl](index.html) module"]
pub struct CH2_CNTRL_SPEC;
impl crate::RegisterSpec for CH2_CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_cntrl::R](R) reader structure"]
impl crate::Readable for CH2_CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_cntrl::W](W) writer structure"]
impl crate::Writable for CH2_CNTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2_CNTRL to value 0"]
impl crate::Resettable for CH2_CNTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
