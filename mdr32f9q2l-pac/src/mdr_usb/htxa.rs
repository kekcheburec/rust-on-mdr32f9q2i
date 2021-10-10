#[doc = "Register `HTXA` reader"]
pub struct R(crate::R<HTXA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXA` writer"]
pub struct W(crate::W<HTXA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXA_SPEC>;
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
impl From<crate::W<HTXA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVADDR` reader - "]
pub struct DEVADDR_R(crate::FieldReader<u8, u8>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - "]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxa](index.html) module"]
pub struct HTXA_SPEC;
impl crate::RegisterSpec for HTXA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxa::R](R) reader structure"]
impl crate::Readable for HTXA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxa::W](W) writer structure"]
impl crate::Writable for HTXA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXA to value 0"]
impl crate::Resettable for HTXA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
