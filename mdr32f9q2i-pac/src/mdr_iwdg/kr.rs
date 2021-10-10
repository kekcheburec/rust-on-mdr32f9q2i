#[doc = "Register `KR` reader"]
pub struct R(crate::R<KR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KR` writer"]
pub struct W(crate::W<KR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KR_SPEC>;
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
impl From<crate::W<KR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - "]
pub struct KEY_R(crate::FieldReader<u16, u16>);
impl KEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        KEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY` writer - "]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
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
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IWDG Key Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kr](index.html) module"]
pub struct KR_SPEC;
impl crate::RegisterSpec for KR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [kr::R](R) reader structure"]
impl crate::Readable for KR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kr::W](W) writer structure"]
impl crate::Writable for KR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KR to value 0"]
impl crate::Resettable for KR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
