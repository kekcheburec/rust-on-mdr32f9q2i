#[doc = "Register `SEP1_NTS` reader"]
pub struct R(crate::R<SEP1_NTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP1_NTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP1_NTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP1_NTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP1_NTS` writer"]
pub struct W(crate::W<SEP1_NTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP1_NTS_SPEC>;
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
impl From<crate::W<SEP1_NTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP1_NTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NTTYPE` reader - "]
pub struct NTTYPE_R(crate::FieldReader<u8, u8>);
impl NTTYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        NTTYPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NTTYPE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NTTYPE` writer - "]
pub struct NTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> NTTYPE_W<'a> {
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
    pub fn nttype(&self) -> NTTYPE_R {
        NTTYPE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn nttype(&mut self) -> NTTYPE_W {
        NTTYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep1_nts](index.html) module"]
pub struct SEP1_NTS_SPEC;
impl crate::RegisterSpec for SEP1_NTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep1_nts::R](R) reader structure"]
impl crate::Readable for SEP1_NTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep1_nts::W](W) writer structure"]
impl crate::Writable for SEP1_NTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP1_NTS to value 0"]
impl crate::Resettable for SEP1_NTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
