#[doc = "Register `TX` reader"]
pub struct R(crate::R<TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX` writer"]
pub struct W(crate::W<TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SPEC>;
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
impl From<crate::W<TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX` reader - "]
pub struct TX_R(crate::FieldReader<u32, u32>);
impl TX_R {
    pub(crate) fn new(bits: u32) -> Self {
        TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX` writer - "]
pub struct TX_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_W<'a> {
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
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx](index.html) module"]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx::R](R) reader structure"]
impl crate::Readable for TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx::W](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TX to value 0"]
impl crate::Resettable for TX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
