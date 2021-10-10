#[doc = "Register `SEP_FIFO3_RXFD` reader"]
pub struct R(crate::R<SEP_FIFO3_RXFD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP_FIFO3_RXFD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP_FIFO3_RXFD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP_FIFO3_RXFD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP_FIFO3_RXFD` writer"]
pub struct W(crate::W<SEP_FIFO3_RXFD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP_FIFO3_RXFD_SPEC>;
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
impl From<crate::W<SEP_FIFO3_RXFD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP_FIFO3_RXFD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFODATA` reader - "]
pub struct RXFIFODATA_R(crate::FieldReader<u8, u8>);
impl RXFIFODATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXFIFODATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFODATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFODATA` writer - "]
pub struct RXFIFODATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFODATA_W<'a> {
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
    pub fn rxfifodata(&self) -> RXFIFODATA_R {
        RXFIFODATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rxfifodata(&mut self) -> RXFIFODATA_W {
        RXFIFODATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep_fifo3_rxfd](index.html) module"]
pub struct SEP_FIFO3_RXFD_SPEC;
impl crate::RegisterSpec for SEP_FIFO3_RXFD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep_fifo3_rxfd::R](R) reader structure"]
impl crate::Readable for SEP_FIFO3_RXFD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep_fifo3_rxfd::W](W) writer structure"]
impl crate::Writable for SEP_FIFO3_RXFD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP_FIFO3_RXFD to value 0"]
impl crate::Resettable for SEP_FIFO3_RXFD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
