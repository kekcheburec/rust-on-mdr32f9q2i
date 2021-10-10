#[doc = "Register `HFN_L` reader"]
pub struct R(crate::R<HFN_L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFN_L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFN_L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFN_L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFN_L` writer"]
pub struct W(crate::W<HFN_L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFN_L_SPEC>;
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
impl From<crate::W<HFN_L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFN_L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FNUM` reader - "]
pub struct FNUM_R(crate::FieldReader<u8, u8>);
impl FNUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        FNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNUM` writer - "]
pub struct FNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> FNUM_W<'a> {
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
    pub fn fnum(&self) -> FNUM_R {
        FNUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn fnum(&mut self) -> FNUM_W {
        FNUM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfn_l](index.html) module"]
pub struct HFN_L_SPEC;
impl crate::RegisterSpec for HFN_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfn_l::R](R) reader structure"]
impl crate::Readable for HFN_L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfn_l::W](W) writer structure"]
impl crate::Writable for HFN_L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFN_L to value 0"]
impl crate::Resettable for HFN_L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
