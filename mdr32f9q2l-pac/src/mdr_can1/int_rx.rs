#[doc = "Register `INT_RX` reader"]
pub struct R(crate::R<INT_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_RX` writer"]
pub struct W(crate::W<INT_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RX_SPEC>;
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
impl From<crate::W<INT_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_RX` reader - "]
pub struct INT_RX_R(crate::FieldReader<u32, u32>);
impl INT_RX_R {
    pub(crate) fn new(bits: u32) -> Self {
        INT_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_RX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_RX` writer - "]
pub struct INT_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_RX_W<'a> {
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
    pub fn int_rx(&self) -> INT_RX_R {
        INT_RX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_rx(&mut self) -> INT_RX_W {
        INT_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_rx](index.html) module"]
pub struct INT_RX_SPEC;
impl crate::RegisterSpec for INT_RX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_rx::R](R) reader structure"]
impl crate::Readable for INT_RX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_rx::W](W) writer structure"]
impl crate::Writable for INT_RX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_RX to value 0"]
impl crate::Resettable for INT_RX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
