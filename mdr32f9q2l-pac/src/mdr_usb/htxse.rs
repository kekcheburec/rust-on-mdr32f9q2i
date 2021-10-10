#[doc = "Register `HTXSE` reader"]
pub struct R(crate::R<HTXSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXSE` writer"]
pub struct W(crate::W<HTXSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXSE_SPEC>;
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
impl From<crate::W<HTXSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFEN` reader - "]
pub struct SOFEN_R(crate::FieldReader<bool, bool>);
impl SOFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFEN` writer - "]
pub struct SOFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFEN_W<'a> {
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
    pub fn sofen(&self) -> SOFEN_R {
        SOFEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sofen(&mut self) -> SOFEN_W {
        SOFEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxse](index.html) module"]
pub struct HTXSE_SPEC;
impl crate::RegisterSpec for HTXSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxse::R](R) reader structure"]
impl crate::Readable for HTXSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxse::W](W) writer structure"]
impl crate::Writable for HTXSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXSE to value 0"]
impl crate::Resettable for HTXSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
