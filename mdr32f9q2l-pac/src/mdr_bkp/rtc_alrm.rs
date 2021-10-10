#[doc = "Register `RTC_ALRM` reader"]
pub struct R(crate::R<RTC_ALRM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_ALRM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_ALRM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_ALRM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_ALRM` writer"]
pub struct W(crate::W<RTC_ALRM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_ALRM_SPEC>;
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
impl From<crate::W<RTC_ALRM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_ALRM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ALRM` reader - "]
pub struct RTC_ALRM_R(crate::FieldReader<u32, u32>);
impl RTC_ALRM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RTC_ALRM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ALRM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_ALRM` writer - "]
pub struct RTC_ALRM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ALRM_W<'a> {
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
    pub fn rtc_alrm(&self) -> RTC_ALRM_R {
        RTC_ALRM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rtc_alrm(&mut self) -> RTC_ALRM_W {
        RTC_ALRM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Realtime Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_alrm](index.html) module"]
pub struct RTC_ALRM_SPEC;
impl crate::RegisterSpec for RTC_ALRM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_alrm::R](R) reader structure"]
impl crate::Readable for RTC_ALRM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_alrm::W](W) writer structure"]
impl crate::Writable for RTC_ALRM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_ALRM to value 0"]
impl crate::Resettable for RTC_ALRM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
