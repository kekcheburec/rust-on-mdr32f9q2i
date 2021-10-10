#[doc = "Register `CH2_CNTRL2` reader"]
pub struct R(crate::R<CH2_CNTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_CNTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_CNTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_CNTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_CNTRL2` writer"]
pub struct W(crate::W<CH2_CNTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_CNTRL2_SPEC>;
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
impl From<crate::W<CH2_CNTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_CNTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHSEL1` reader - "]
pub struct CHSEL1_R(crate::FieldReader<u8, u8>);
impl CHSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL1` writer - "]
pub struct CHSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `CCR1_EN` reader - "]
pub struct CCR1_EN_R(crate::FieldReader<bool, bool>);
impl CCR1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCR1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR1_EN` writer - "]
pub struct CCR1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CCRRLD` reader - "]
pub struct CCRRLD_R(crate::FieldReader<bool, bool>);
impl CCRRLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCRRLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCRRLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCRRLD` writer - "]
pub struct CCRRLD_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRRLD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chsel1(&self) -> CHSEL1_R {
        CHSEL1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ccr1_en(&self) -> CCR1_EN_R {
        CCR1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccrrld(&self) -> CCRRLD_R {
        CCRRLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn chsel1(&mut self) -> CHSEL1_W {
        CHSEL1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ccr1_en(&mut self) -> CCR1_EN_W {
        CCR1_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ccrrld(&mut self) -> CCRRLD_W {
        CCRRLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cntrl2](index.html) module"]
pub struct CH2_CNTRL2_SPEC;
impl crate::RegisterSpec for CH2_CNTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_cntrl2::R](R) reader structure"]
impl crate::Readable for CH2_CNTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_cntrl2::W](W) writer structure"]
impl crate::Writable for CH2_CNTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2_CNTRL2 to value 0"]
impl crate::Resettable for CH2_CNTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
