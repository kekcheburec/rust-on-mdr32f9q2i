#[doc = "Register `BUF_19_DLC` reader"]
pub struct R(crate::R<BUF_19_DLC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_19_DLC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_19_DLC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_19_DLC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_19_DLC` writer"]
pub struct W(crate::W<BUF_19_DLC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_19_DLC_SPEC>;
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
impl From<crate::W<BUF_19_DLC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_19_DLC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - "]
pub struct DLC_R(crate::FieldReader<u8, u8>);
impl DLC_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLC` writer - "]
pub struct DLC_W<'a> {
    w: &'a mut W,
}
impl<'a> DLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `RTR` reader - "]
pub struct RTR_R(crate::FieldReader<bool, bool>);
impl RTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR` writer - "]
pub struct RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_W<'a> {
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
#[doc = "Field `R1` reader - "]
pub struct R1_R(crate::FieldReader<bool, bool>);
impl R1_R {
    pub(crate) fn new(bits: bool) -> Self {
        R1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R1` writer - "]
pub struct R1_W<'a> {
    w: &'a mut W,
}
impl<'a> R1_W<'a> {
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
#[doc = "Field `R0` reader - "]
pub struct R0_R(crate::FieldReader<bool, bool>);
impl R0_R {
    pub(crate) fn new(bits: bool) -> Self {
        R0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0` writer - "]
pub struct R0_W<'a> {
    w: &'a mut W,
}
impl<'a> R0_W<'a> {
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
#[doc = "Field `SSR` reader - "]
pub struct SSR_R(crate::FieldReader<bool, bool>);
impl SSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSR` writer - "]
pub struct SSR_W<'a> {
    w: &'a mut W,
}
impl<'a> SSR_W<'a> {
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
#[doc = "Field `IDE` reader - "]
pub struct IDE_R(crate::FieldReader<bool, bool>);
impl IDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDE` writer - "]
pub struct IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_W<'a> {
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
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r1(&self) -> R1_R {
        R1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r0(&self) -> R0_R {
        R0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ssr(&self) -> SSR_R {
        SSR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rtr(&mut self) -> RTR_W {
        RTR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn r1(&mut self) -> R1_W {
        R1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn r0(&mut self) -> R0_W {
        R0_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ssr(&mut self) -> SSR_W {
        SSR_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ide(&mut self) -> IDE_W {
        IDE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer DLC Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_19_dlc](index.html) module"]
pub struct BUF_19_DLC_SPEC;
impl crate::RegisterSpec for BUF_19_DLC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_19_dlc::R](R) reader structure"]
impl crate::Readable for BUF_19_DLC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_19_dlc::W](W) writer structure"]
impl crate::Writable for BUF_19_DLC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_19_DLC to value 0"]
impl crate::Resettable for BUF_19_DLC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
