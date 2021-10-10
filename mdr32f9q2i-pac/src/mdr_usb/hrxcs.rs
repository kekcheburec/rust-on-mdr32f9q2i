#[doc = "Register `HRXCS` reader"]
pub struct R(crate::R<HRXCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXCS` writer"]
pub struct W(crate::W<HRXCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXCS_SPEC>;
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
impl From<crate::W<HRXCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXLS` reader - "]
pub struct RXLS_R(crate::FieldReader<u8, u8>);
impl RXLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLS` writer - "]
pub struct RXLS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rxls(&self) -> RXLS_R {
        RXLS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rxls(&mut self) -> RXLS_W {
        RXLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxcs](index.html) module"]
pub struct HRXCS_SPEC;
impl crate::RegisterSpec for HRXCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxcs::R](R) reader structure"]
impl crate::Readable for HRXCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxcs::W](W) writer structure"]
impl crate::Writable for HRXCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRXCS to value 0"]
impl crate::Resettable for HRXCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
