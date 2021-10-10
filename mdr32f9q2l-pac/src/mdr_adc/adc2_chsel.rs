#[doc = "Register `ADC2_CHSEL` reader"]
pub struct R(crate::R<ADC2_CHSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_CHSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_CHSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_CHSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_CHSEL` writer"]
pub struct W(crate::W<ADC2_CHSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_CHSEL_SPEC>;
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
impl From<crate::W<ADC2_CHSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_CHSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Sl_Ch_Ch_REF` reader - "]
pub struct SL_CH_CH_REF_R(crate::FieldReader<u32, u32>);
impl SL_CH_CH_REF_R {
    pub(crate) fn new(bits: u32) -> Self {
        SL_CH_CH_REF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SL_CH_CH_REF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Sl_Ch_Ch_REF` writer - "]
pub struct SL_CH_CH_REF_W<'a> {
    w: &'a mut W,
}
impl<'a> SL_CH_CH_REF_W<'a> {
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
    pub fn sl_ch_ch_ref(&self) -> SL_CH_CH_REF_R {
        SL_CH_CH_REF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sl_ch_ch_ref(&mut self) -> SL_CH_CH_REF_W {
        SL_CH_CH_REF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 Channel Selector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_chsel](index.html) module"]
pub struct ADC2_CHSEL_SPEC;
impl crate::RegisterSpec for ADC2_CHSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_chsel::R](R) reader structure"]
impl crate::Readable for ADC2_CHSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_chsel::W](W) writer structure"]
impl crate::Writable for ADC2_CHSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2_CHSEL to value 0"]
impl crate::Resettable for ADC2_CHSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
