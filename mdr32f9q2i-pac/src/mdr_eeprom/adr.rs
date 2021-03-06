#[doc = "Register `ADR` reader"]
pub struct R(crate::R<ADR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADR` writer"]
pub struct W(crate::W<ADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADR_SPEC>;
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
impl From<crate::W<ADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADR` reader - "]
pub struct ADR_R(crate::FieldReader<u32, u32>);
impl ADR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADR` writer - "]
pub struct ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn adr(&mut self) -> ADR_W {
        ADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr](index.html) module"]
pub struct ADR_SPEC;
impl crate::RegisterSpec for ADR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adr::R](R) reader structure"]
impl crate::Readable for ADR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adr::W](W) writer structure"]
impl crate::Writable for ADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADR to value 0"]
impl crate::Resettable for ADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
