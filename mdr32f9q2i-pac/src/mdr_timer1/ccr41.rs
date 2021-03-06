#[doc = "Register `CCR41` reader"]
pub struct R(crate::R<CCR41_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR41_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR41_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR41_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR41` writer"]
pub struct W(crate::W<CCR41_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR41_SPEC>;
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
impl From<crate::W<CCR41_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR41_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR` reader - "]
pub struct CCR_R(crate::FieldReader<u16, u16>);
impl CCR_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR` writer - "]
pub struct CCR_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ccr(&self) -> CCR_R {
        CCR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ccr(&mut self) -> CCR_W {
        CCR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Capture/Compare1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr41](index.html) module"]
pub struct CCR41_SPEC;
impl crate::RegisterSpec for CCR41_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr41::R](R) reader structure"]
impl crate::Readable for CCR41_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr41::W](W) writer structure"]
impl crate::Writable for CCR41_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCR41 to value 0"]
impl crate::Resettable for CCR41_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
