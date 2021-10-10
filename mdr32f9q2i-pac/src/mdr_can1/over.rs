#[doc = "Register `OVER` reader"]
pub struct R(crate::R<OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVER` writer"]
pub struct W(crate::W<OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVER_SPEC>;
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
impl From<crate::W<OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERROR_MAX` reader - "]
pub struct ERROR_MAX_R(crate::FieldReader<u8, u8>);
impl ERROR_MAX_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERROR_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_MAX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_MAX` writer - "]
pub struct ERROR_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_MAX_W<'a> {
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
    pub fn error_max(&self) -> ERROR_MAX_R {
        ERROR_MAX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn error_max(&mut self) -> ERROR_MAX_W {
        ERROR_MAX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [over](index.html) module"]
pub struct OVER_SPEC;
impl crate::RegisterSpec for OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [over::R](R) reader structure"]
impl crate::Readable for OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [over::W](W) writer structure"]
impl crate::Writable for OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVER to value 0"]
impl crate::Resettable for OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
