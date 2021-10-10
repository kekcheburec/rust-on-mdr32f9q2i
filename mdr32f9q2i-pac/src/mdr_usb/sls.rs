#[doc = "Register `SLS` reader"]
pub struct R(crate::R<SLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLS` writer"]
pub struct W(crate::W<SLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLS_SPEC>;
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
impl From<crate::W<SLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRXLS` reader - "]
pub struct SCRXLS_R(crate::FieldReader<u8, u8>);
impl SCRXLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCRXLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRXLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRXLS` writer - "]
pub struct SCRXLS_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRXLS_W<'a> {
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
    pub fn scrxls(&self) -> SCRXLS_R {
        SCRXLS_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn scrxls(&mut self) -> SCRXLS_W {
        SCRXLS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sls](index.html) module"]
pub struct SLS_SPEC;
impl crate::RegisterSpec for SLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sls::R](R) reader structure"]
impl crate::Readable for SLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sls::W](W) writer structure"]
impl crate::Writable for SLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLS to value 0"]
impl crate::Resettable for SLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
