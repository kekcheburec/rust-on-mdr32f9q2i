#[doc = "Register `ADC1_RESULT` reader"]
pub struct R(crate::R<ADC1_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1_RESULT` writer"]
pub struct W(crate::W<ADC1_RESULT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1_RESULT_SPEC>;
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
impl From<crate::W<ADC1_RESULT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1_RESULT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT` reader - "]
pub struct RESULT_R(crate::FieldReader<u16, u16>);
impl RESULT_R {
    pub(crate) fn new(bits: u16) -> Self {
        RESULT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESULT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESULT` writer - "]
pub struct RESULT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESULT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `CHANNEL` reader - "]
pub struct CHANNEL_R(crate::FieldReader<u16, u16>);
impl CHANNEL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHANNEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHANNEL` writer - "]
pub struct CHANNEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHANNEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn result(&mut self) -> RESULT_W {
        RESULT_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn channel(&mut self) -> CHANNEL_W {
        CHANNEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1 Result Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1_result](index.html) module"]
pub struct ADC1_RESULT_SPEC;
impl crate::RegisterSpec for ADC1_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1_result::R](R) reader structure"]
impl crate::Readable for ADC1_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1_result::W](W) writer structure"]
impl crate::Writable for ADC1_RESULT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC1_RESULT to value 0"]
impl crate::Resettable for ADC1_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
