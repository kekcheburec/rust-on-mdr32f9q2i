#[doc = "Register `CHNL_SW_REQUEST` reader"]
pub struct R(crate::R<CHNL_SW_REQUEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHNL_SW_REQUEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHNL_SW_REQUEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHNL_SW_REQUEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHNL_SW_REQUEST` writer"]
pub struct W(crate::W<CHNL_SW_REQUEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHNL_SW_REQUEST_SPEC>;
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
impl From<crate::W<CHNL_SW_REQUEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHNL_SW_REQUEST_SPEC>) -> Self {
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
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chnl_sw_request](index.html) module"]
pub struct CHNL_SW_REQUEST_SPEC;
impl crate::RegisterSpec for CHNL_SW_REQUEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chnl_sw_request::R](R) reader structure"]
impl crate::Readable for CHNL_SW_REQUEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chnl_sw_request::W](W) writer structure"]
impl crate::Writable for CHNL_SW_REQUEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHNL_SW_REQUEST to value 0"]
impl crate::Resettable for CHNL_SW_REQUEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
