#[doc = "Register `HS_CONTROL` reader"]
pub struct R(crate::R<HS_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HS_CONTROL` writer"]
pub struct W(crate::W<HS_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_CONTROL_SPEC>;
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
impl From<crate::W<HS_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSE_ON` reader - "]
pub struct HSE_ON_R(crate::FieldReader<bool, bool>);
impl HSE_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSE_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE_ON` writer - "]
pub struct HSE_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_ON_W<'a> {
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
#[doc = "Field `HSE_BYP` reader - "]
pub struct HSE_BYP_R(crate::FieldReader<bool, bool>);
impl HSE_BYP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSE_BYP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSE_BYP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSE_BYP` writer - "]
pub struct HSE_BYP_W<'a> {
    w: &'a mut W,
}
impl<'a> HSE_BYP_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hse_on(&self) -> HSE_ON_R {
        HSE_ON_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hse_byp(&self) -> HSE_BYP_R {
        HSE_BYP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hse_on(&mut self) -> HSE_ON_W {
        HSE_ON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hse_byp(&mut self) -> HSE_BYP_W {
        HSE_BYP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs_control](index.html) module"]
pub struct HS_CONTROL_SPEC;
impl crate::RegisterSpec for HS_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs_control::R](R) reader structure"]
impl crate::Readable for HS_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs_control::W](W) writer structure"]
impl crate::Writable for HS_CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HS_CONTROL to value 0"]
impl crate::Resettable for HS_CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
