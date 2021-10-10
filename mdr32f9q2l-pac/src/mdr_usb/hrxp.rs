#[doc = "Register `HRXP` reader"]
pub struct R(crate::R<HRXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRXP` writer"]
pub struct W(crate::W<HRXP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRXP_SPEC>;
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
impl From<crate::W<HRXP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRXP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPID` reader - "]
pub struct RPID_R(crate::FieldReader<u8, u8>);
impl RPID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RPID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RPID` writer - "]
pub struct RPID_W<'a> {
    w: &'a mut W,
}
impl<'a> RPID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpid(&self) -> RPID_R {
        RPID_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn rpid(&mut self) -> RPID_W {
        RPID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrxp](index.html) module"]
pub struct HRXP_SPEC;
impl crate::RegisterSpec for HRXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrxp::R](R) reader structure"]
impl crate::Readable for HRXP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrxp::W](W) writer structure"]
impl crate::Writable for HRXP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRXP to value 0"]
impl crate::Resettable for HRXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
