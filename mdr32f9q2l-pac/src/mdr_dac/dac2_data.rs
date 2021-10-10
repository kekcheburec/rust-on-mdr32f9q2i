#[doc = "Register `DAC2_DATA` reader"]
pub struct R(crate::R<DAC2_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC2_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC2_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC2_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC2_DATA` writer"]
pub struct W(crate::W<DAC2_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC2_DATA_SPEC>;
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
impl From<crate::W<DAC2_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC2_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAC1DATA` reader - "]
pub struct DAC1DATA_R(crate::FieldReader<u16, u16>);
impl DAC1DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DAC1DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC1DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC1DATA` writer - "]
pub struct DAC1DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC1DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `DAC0DATA` reader - "]
pub struct DAC0DATA_R(crate::FieldReader<u16, u16>);
impl DAC0DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        DAC0DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAC0DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAC0DATA` writer - "]
pub struct DAC0DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DAC0DATA_W<'a> {
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
    pub fn dac1data(&self) -> DAC1DATA_R {
        DAC1DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dac0data(&self) -> DAC0DATA_R {
        DAC0DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn dac1data(&mut self) -> DAC1DATA_W {
        DAC1DATA_W { w: self }
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn dac0data(&mut self) -> DAC0DATA_W {
        DAC0DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC2 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac2_data](index.html) module"]
pub struct DAC2_DATA_SPEC;
impl crate::RegisterSpec for DAC2_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac2_data::R](R) reader structure"]
impl crate::Readable for DAC2_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac2_data::W](W) writer structure"]
impl crate::Writable for DAC2_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC2_DATA to value 0"]
impl crate::Resettable for DAC2_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
