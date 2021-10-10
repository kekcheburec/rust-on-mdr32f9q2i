#[doc = "Register `ADC2_H_LEVEL` reader"]
pub struct R(crate::R<ADC2_H_LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_H_LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_H_LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_H_LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_H_LEVEL` writer"]
pub struct W(crate::W<ADC2_H_LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_H_LEVEL_SPEC>;
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
impl From<crate::W<ADC2_H_LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_H_LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REG_H_LEVEL` reader - "]
pub struct REG_H_LEVEL_R(crate::FieldReader<u16, u16>);
impl REG_H_LEVEL_R {
    pub(crate) fn new(bits: u16) -> Self {
        REG_H_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_H_LEVEL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REG_H_LEVEL` writer - "]
pub struct REG_H_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_H_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_h_level(&self) -> REG_H_LEVEL_R {
        REG_H_LEVEL_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn reg_h_level(&mut self) -> REG_H_LEVEL_W {
        REG_H_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 High Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_h_level](index.html) module"]
pub struct ADC2_H_LEVEL_SPEC;
impl crate::RegisterSpec for ADC2_H_LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_h_level::R](R) reader structure"]
impl crate::Readable for ADC2_H_LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_h_level::W](W) writer structure"]
impl crate::Writable for ADC2_H_LEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2_H_LEVEL to value 0"]
impl crate::Resettable for ADC2_H_LEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
