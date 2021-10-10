#[doc = "Register `HSTM` reader"]
pub struct R(crate::R<HSTM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTM` writer"]
pub struct W(crate::W<HSTM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTM_SPEC>;
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
impl From<crate::W<HSTM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSTM` reader - "]
pub struct HSTM_R(crate::FieldReader<u8, u8>);
impl HSTM_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSTM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSTM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSTM` writer - "]
pub struct HSTM_W<'a> {
    w: &'a mut W,
}
impl<'a> HSTM_W<'a> {
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
    pub fn hstm(&self) -> HSTM_R {
        HSTM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hstm(&mut self) -> HSTM_W {
        HSTM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstm](index.html) module"]
pub struct HSTM_SPEC;
impl crate::RegisterSpec for HSTM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstm::R](R) reader structure"]
impl crate::Readable for HSTM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstm::W](W) writer structure"]
impl crate::Writable for HSTM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTM to value 0"]
impl crate::Resettable for HSTM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
