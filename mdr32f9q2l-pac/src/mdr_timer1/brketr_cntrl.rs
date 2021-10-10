#[doc = "Register `BRKETR_CNTRL` reader"]
pub struct R(crate::R<BRKETR_CNTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRKETR_CNTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRKETR_CNTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRKETR_CNTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRKETR_CNTRL` writer"]
pub struct W(crate::W<BRKETR_CNTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRKETR_CNTRL_SPEC>;
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
impl From<crate::W<BRKETR_CNTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRKETR_CNTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK_INV` reader - "]
pub struct BRK_INV_R(crate::FieldReader<bool, bool>);
impl BRK_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_INV` writer - "]
pub struct BRK_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_INV_W<'a> {
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
#[doc = "Field `ETR_INV` reader - "]
pub struct ETR_INV_R(crate::FieldReader<bool, bool>);
impl ETR_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETR_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_INV` writer - "]
pub struct ETR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_INV_W<'a> {
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
#[doc = "Field `ETR_PSC` reader - "]
pub struct ETR_PSC_R(crate::FieldReader<u8, u8>);
impl ETR_PSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETR_PSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_PSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_PSC` writer - "]
pub struct ETR_PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `ETR_FILTER` reader - "]
pub struct ETR_FILTER_R(crate::FieldReader<u8, u8>);
impl ETR_FILTER_R {
    pub(crate) fn new(bits: u8) -> Self {
        ETR_FILTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_FILTER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_FILTER` writer - "]
pub struct ETR_FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_FILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn brk_inv(&self) -> BRK_INV_R {
        BRK_INV_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn etr_inv(&self) -> ETR_INV_R {
        ETR_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn etr_psc(&self) -> ETR_PSC_R {
        ETR_PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn etr_filter(&self) -> ETR_FILTER_R {
        ETR_FILTER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn brk_inv(&mut self) -> BRK_INV_W {
        BRK_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn etr_inv(&mut self) -> ETR_INV_W {
        ETR_INV_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn etr_psc(&mut self) -> ETR_PSC_W {
        ETR_PSC_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn etr_filter(&mut self) -> ETR_FILTER_W {
        ETR_FILTER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer BRK/ETR Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brketr_cntrl](index.html) module"]
pub struct BRKETR_CNTRL_SPEC;
impl crate::RegisterSpec for BRKETR_CNTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brketr_cntrl::R](R) reader structure"]
impl crate::Readable for BRKETR_CNTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brketr_cntrl::W](W) writer structure"]
impl crate::Writable for BRKETR_CNTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRKETR_CNTRL to value 0"]
impl crate::Resettable for BRKETR_CNTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
