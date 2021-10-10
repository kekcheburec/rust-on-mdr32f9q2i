#[doc = "Register `CH2_DTG` reader"]
pub struct R(crate::R<CH2_DTG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_DTG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_DTG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_DTG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_DTG` writer"]
pub struct W(crate::W<CH2_DTG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_DTG_SPEC>;
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
impl From<crate::W<CH2_DTG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_DTG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGx` reader - "]
pub struct DTGX_R(crate::FieldReader<u8, u8>);
impl DTGX_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTGX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTGX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTGx` writer - "]
pub struct DTGX_W<'a> {
    w: &'a mut W,
}
impl<'a> DTGX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `EDTS` reader - "]
pub struct EDTS_R(crate::FieldReader<bool, bool>);
impl EDTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EDTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EDTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDTS` writer - "]
pub struct EDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EDTS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DTG` reader - "]
pub struct DTG_R(crate::FieldReader<u8, u8>);
impl DTG_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTG` writer - "]
pub struct DTG_W<'a> {
    w: &'a mut W,
}
impl<'a> DTG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dtgx(&self) -> DTGX_R {
        DTGX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn edts(&self) -> EDTS_R {
        EDTS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dtgx(&mut self) -> DTGX_W {
        DTGX_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn edts(&mut self) -> EDTS_W {
        EDTS_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dtg(&mut self) -> DTG_W {
        DTG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel DTG Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dtg](index.html) module"]
pub struct CH2_DTG_SPEC;
impl crate::RegisterSpec for CH2_DTG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_dtg::R](R) reader structure"]
impl crate::Readable for CH2_DTG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_dtg::W](W) writer structure"]
impl crate::Writable for CH2_DTG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2_DTG to value 0"]
impl crate::Resettable for CH2_DTG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
