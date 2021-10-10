#[doc = "Register `HTXT` reader"]
pub struct R(crate::R<HTXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXT` writer"]
pub struct W(crate::W<HTXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXT_SPEC>;
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
impl From<crate::W<HTXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TTYPE` reader - "]
pub struct TTYPE_R(crate::FieldReader<u8, u8>);
impl TTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TTYPE` writer - "]
pub struct TTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TTYPE_W<'a> {
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
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn ttype(&mut self) -> TTYPE_W {
        TTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB HTXT Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxt](index.html) module"]
pub struct HTXT_SPEC;
impl crate::RegisterSpec for HTXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxt::R](R) reader structure"]
impl crate::Readable for HTXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxt::W](W) writer structure"]
impl crate::Writable for HTXT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXT to value 0"]
impl crate::Resettable for HTXT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
