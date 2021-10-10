#[doc = "Register `ADC2_STATUS` reader"]
pub struct R(crate::R<ADC2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC2_STATUS` writer"]
pub struct W(crate::W<ADC2_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC2_STATUS_SPEC>;
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
impl From<crate::W<ADC2_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC2_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Flg_REG_OVERWRITE` reader - "]
pub struct FLG_REG_OVERWRITE_R(crate::FieldReader<bool, bool>);
impl FLG_REG_OVERWRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLG_REG_OVERWRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLG_REG_OVERWRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Flg_REG_OVERWRITE` writer - "]
pub struct FLG_REG_OVERWRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLG_REG_OVERWRITE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `Flg_REG_AWOIFEN` reader - "]
pub struct FLG_REG_AWOIFEN_R(crate::FieldReader<bool, bool>);
impl FLG_REG_AWOIFEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLG_REG_AWOIFEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLG_REG_AWOIFEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Flg_REG_AWOIFEN` writer - "]
pub struct FLG_REG_AWOIFEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLG_REG_AWOIFEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `Flg_REG_EOCIF` reader - "]
pub struct FLG_REG_EOCIF_R(crate::FieldReader<bool, bool>);
impl FLG_REG_EOCIF_R {
    pub(crate) fn new(bits: bool) -> Self {
        FLG_REG_EOCIF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLG_REG_EOCIF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Flg_REG_EOCIF` writer - "]
pub struct FLG_REG_EOCIF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLG_REG_EOCIF_W<'a> {
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
#[doc = "Field `AWOIF_IE` reader - "]
pub struct AWOIF_IE_R(crate::FieldReader<bool, bool>);
impl AWOIF_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AWOIF_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AWOIF_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AWOIF_IE` writer - "]
pub struct AWOIF_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> AWOIF_IE_W<'a> {
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
#[doc = "Field `ECOIF_IE` reader - "]
pub struct ECOIF_IE_R(crate::FieldReader<bool, bool>);
impl ECOIF_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ECOIF_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECOIF_IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECOIF_IE` writer - "]
pub struct ECOIF_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ECOIF_IE_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flg_reg_overwrite(&self) -> FLG_REG_OVERWRITE_R {
        FLG_REG_OVERWRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flg_reg_awoifen(&self) -> FLG_REG_AWOIFEN_R {
        FLG_REG_AWOIFEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flg_reg_eocif(&self) -> FLG_REG_EOCIF_R {
        FLG_REG_EOCIF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awoif_ie(&self) -> AWOIF_IE_R {
        AWOIF_IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ecoif_ie(&self) -> ECOIF_IE_R {
        ECOIF_IE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flg_reg_overwrite(&mut self) -> FLG_REG_OVERWRITE_W {
        FLG_REG_OVERWRITE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flg_reg_awoifen(&mut self) -> FLG_REG_AWOIFEN_W {
        FLG_REG_AWOIFEN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flg_reg_eocif(&mut self) -> FLG_REG_EOCIF_W {
        FLG_REG_EOCIF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn awoif_ie(&mut self) -> AWOIF_IE_W {
        AWOIF_IE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ecoif_ie(&mut self) -> ECOIF_IE_W {
        ECOIF_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC2 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2_status](index.html) module"]
pub struct ADC2_STATUS_SPEC;
impl crate::RegisterSpec for ADC2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc2_status::R](R) reader structure"]
impl crate::Readable for ADC2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc2_status::W](W) writer structure"]
impl crate::Writable for ADC2_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC2_STATUS to value 0"]
impl crate::Resettable for ADC2_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
