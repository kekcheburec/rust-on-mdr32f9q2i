#[doc = "Register `CNTRL` reader"]
pub struct R(crate::R<CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTRL` writer"]
pub struct W(crate::W<CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTRL_SPEC>;
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
impl From<crate::W<CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_EN` reader - "]
pub struct CNT_EN_R(crate::FieldReader<bool, bool>);
impl CNT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_EN` writer - "]
pub struct CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_EN_W<'a> {
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
#[doc = "Field `ARRB_EN` reader - "]
pub struct ARRB_EN_R(crate::FieldReader<bool, bool>);
impl ARRB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARRB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARRB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARRB_EN` writer - "]
pub struct ARRB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARRB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `DIR` reader - "]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` writer - "]
pub struct DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIR_W<'a> {
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
#[doc = "Field `FDTS` reader - "]
pub struct FDTS_R(crate::FieldReader<u8, u8>);
impl FDTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FDTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDTS` writer - "]
pub struct FDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> FDTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `CNT_MODE` reader - "]
pub struct CNT_MODE_R(crate::FieldReader<u8, u8>);
impl CNT_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_MODE` writer - "]
pub struct CNT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `EVENT_SEL` reader - "]
pub struct EVENT_SEL_R(crate::FieldReader<u8, u8>);
impl EVENT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EVENT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENT_SEL` writer - "]
pub struct EVENT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENT_SEL_W<'a> {
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
    pub fn cnt_en(&self) -> CNT_EN_R {
        CNT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arrb_en(&self) -> ARRB_EN_R {
        ARRB_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wr_cmpl(&self) -> WR_CMPL_R {
        WR_CMPL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fdts(&self) -> FDTS_R {
        FDTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cnt_mode(&self) -> CNT_MODE_R {
        CNT_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn event_sel(&self) -> EVENT_SEL_R {
        EVENT_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_en(&mut self) -> CNT_EN_W {
        CNT_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn arrb_en(&mut self) -> ARRB_EN_W {
        ARRB_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn wr_cmpl(&mut self) -> WR_CMPL_W {
        WR_CMPL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fdts(&mut self) -> FDTS_W {
        FDTS_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cnt_mode(&mut self) -> CNT_MODE_W {
        CNT_MODE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn event_sel(&mut self) -> EVENT_SEL_W {
        EVENT_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntrl](index.html) module"]
pub struct CNTRL_SPEC;
impl crate::RegisterSpec for CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntrl::R](R) reader structure"]
impl crate::Readable for CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntrl::W](W) writer structure"]
impl crate::Writable for CNTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTRL to value 0"]
impl crate::Resettable for CNTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
