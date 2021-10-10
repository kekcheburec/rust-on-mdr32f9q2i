#[doc = "Register `BUF_FILTER23_MASK` reader"]
pub struct R(crate::R<BUF_FILTER23_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_FILTER23_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_FILTER23_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_FILTER23_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_FILTER23_MASK` writer"]
pub struct W(crate::W<BUF_FILTER23_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_FILTER23_MASK_SPEC>;
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
impl From<crate::W<BUF_FILTER23_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_FILTER23_MASK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_filter23_mask](index.html) module"]
pub struct BUF_FILTER23_MASK_SPEC;
impl crate::RegisterSpec for BUF_FILTER23_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_filter23_mask::R](R) reader structure"]
impl crate::Readable for BUF_FILTER23_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_filter23_mask::W](W) writer structure"]
impl crate::Writable for BUF_FILTER23_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_FILTER23_MASK to value 0"]
impl crate::Resettable for BUF_FILTER23_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}