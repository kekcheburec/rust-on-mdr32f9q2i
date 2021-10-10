#[doc = "Register `SA` reader"]
pub struct R(crate::R<SA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SA` writer"]
pub struct W(crate::W<SA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SA_SPEC>;
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
impl From<crate::W<SA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCDEVADDR` reader - "]
pub struct SCDEVADDR_R(crate::FieldReader<u8, u8>);
impl SCDEVADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCDEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCDEVADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCDEVADDR` writer - "]
pub struct SCDEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCDEVADDR_W<'a> {
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
    pub fn scdevaddr(&self) -> SCDEVADDR_R {
        SCDEVADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn scdevaddr(&mut self) -> SCDEVADDR_W {
        SCDEVADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sa](index.html) module"]
pub struct SA_SPEC;
impl crate::RegisterSpec for SA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sa::R](R) reader structure"]
impl crate::Readable for SA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sa::W](W) writer structure"]
impl crate::Writable for SA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SA to value 0"]
impl crate::Resettable for SA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
