#[doc = "Register `RXD` reader"]
pub struct R(crate::R<RXD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXD` writer"]
pub struct W(crate::W<RXD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXD_SPEC>;
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
impl From<crate::W<RXD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXD` reader - "]
pub struct RXD_R(crate::FieldReader<u8, u8>);
impl RXD_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXD` writer - "]
pub struct RXD_W<'a> {
    w: &'a mut W,
}
impl<'a> RXD_W<'a> {
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
    pub fn rxd(&self) -> RXD_R {
        RXD_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxd(&mut self) -> RXD_W {
        RXD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Received Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](index.html) module"]
pub struct RXD_SPEC;
impl crate::RegisterSpec for RXD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxd::R](R) reader structure"]
impl crate::Readable for RXD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxd::W](W) writer structure"]
impl crate::Writable for RXD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RXD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
