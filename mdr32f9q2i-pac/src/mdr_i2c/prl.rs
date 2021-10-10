#[doc = "Register `PRL` reader"]
pub struct R(crate::R<PRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRL` writer"]
pub struct W(crate::W<PRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRL_SPEC>;
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
impl From<crate::W<PRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - "]
pub struct PR_R(crate::FieldReader<u8, u8>);
impl PR_R {
    pub(crate) fn new(bits: u8) -> Self {
        PR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PR` writer - "]
pub struct PR_W<'a> {
    w: &'a mut W,
}
impl<'a> PR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W {
        PR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Prescaler (low byte) Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prl](index.html) module"]
pub struct PRL_SPEC;
impl crate::RegisterSpec for PRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prl::R](R) reader structure"]
impl crate::Readable for PRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prl::W](W) writer structure"]
impl crate::Writable for PRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRL to value 0"]
impl crate::Resettable for PRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
