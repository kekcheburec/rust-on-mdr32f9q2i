#[doc = "Register `RESULT_LATCH` reader"]
pub struct R(crate::R<RESULT_LATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESULT_LATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESULT_LATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESULT_LATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESULT_LATCH` writer"]
pub struct W(crate::W<RESULT_LATCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESULT_LATCH_SPEC>;
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
impl From<crate::W<RESULT_LATCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESULT_LATCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Rst_lch` reader - "]
pub struct RST_LCH_R(crate::FieldReader<bool, bool>);
impl RST_LCH_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_LCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_LCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Rst_lch` writer - "]
pub struct RST_LCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RST_LCH_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rst_lch(&self) -> RST_LCH_R {
        RST_LCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rst_lch(&mut self) -> RST_LCH_W {
        RST_LCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "COMP Result Latch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result_latch](index.html) module"]
pub struct RESULT_LATCH_SPEC;
impl crate::RegisterSpec for RESULT_LATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [result_latch::R](R) reader structure"]
impl crate::Readable for RESULT_LATCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [result_latch::W](W) writer structure"]
impl crate::Writable for RESULT_LATCH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESULT_LATCH to value 0"]
impl crate::Resettable for RESULT_LATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
