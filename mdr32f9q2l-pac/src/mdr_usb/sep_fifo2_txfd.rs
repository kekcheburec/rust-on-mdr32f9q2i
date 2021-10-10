#[doc = "Register `SEP_FIFO2_TXFD` reader"]
pub struct R(crate::R<SEP_FIFO2_TXFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP_FIFO2_TXFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP_FIFO2_TXFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP_FIFO2_TXFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP_FIFO2_TXFD` writer"]
pub struct W(crate::W<SEP_FIFO2_TXFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP_FIFO2_TXFD_SPEC>;
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
impl From<crate::W<SEP_FIFO2_TXFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP_FIFO2_TXFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFIFODATA` reader - "]
pub struct TXFIFODATA_R(crate::FieldReader<u8, u8>);
impl TXFIFODATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFODATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFODATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFIFODATA` writer - "]
pub struct TXFIFODATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFODATA_W<'a> {
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
    pub fn txfifodata(&self) -> TXFIFODATA_R {
        TXFIFODATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn txfifodata(&mut self) -> TXFIFODATA_W {
        TXFIFODATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep_fifo2_txfd](index.html) module"]
pub struct SEP_FIFO2_TXFD_SPEC;
impl crate::RegisterSpec for SEP_FIFO2_TXFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep_fifo2_txfd::R](R) reader structure"]
impl crate::Readable for SEP_FIFO2_TXFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep_fifo2_txfd::W](W) writer structure"]
impl crate::Writable for SEP_FIFO2_TXFD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP_FIFO2_TXFD to value 0"]
impl crate::Resettable for SEP_FIFO2_TXFD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
