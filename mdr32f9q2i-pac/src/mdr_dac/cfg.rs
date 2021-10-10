#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Cfg_M_REF0` reader - "]
pub struct CFG_M_REF0_R(crate::FieldReader<bool, bool>);
impl CFG_M_REF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_M_REF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_M_REF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_M_REF0` writer - "]
pub struct CFG_M_REF0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_M_REF0_W<'a> {
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
#[doc = "Field `Cfg_M_REF1` reader - "]
pub struct CFG_M_REF1_R(crate::FieldReader<bool, bool>);
impl CFG_M_REF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_M_REF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_M_REF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_M_REF1` writer - "]
pub struct CFG_M_REF1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_M_REF1_W<'a> {
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
#[doc = "Field `Cfg_ON_DAC0` reader - "]
pub struct CFG_ON_DAC0_R(crate::FieldReader<bool, bool>);
impl CFG_ON_DAC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_ON_DAC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_ON_DAC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_ON_DAC0` writer - "]
pub struct CFG_ON_DAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_ON_DAC0_W<'a> {
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
#[doc = "Field `Cfg_ON_DAC1` reader - "]
pub struct CFG_ON_DAC1_R(crate::FieldReader<bool, bool>);
impl CFG_ON_DAC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_ON_DAC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_ON_DAC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_ON_DAC1` writer - "]
pub struct CFG_ON_DAC1_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_ON_DAC1_W<'a> {
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
#[doc = "Field `Cfg_SYNC_A` reader - "]
pub struct CFG_SYNC_A_R(crate::FieldReader<bool, bool>);
impl CFG_SYNC_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        CFG_SYNC_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFG_SYNC_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Cfg_SYNC_A` writer - "]
pub struct CFG_SYNC_A_W<'a> {
    w: &'a mut W,
}
impl<'a> CFG_SYNC_A_W<'a> {
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
    pub fn cfg_m_ref0(&self) -> CFG_M_REF0_R {
        CFG_M_REF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_m_ref1(&self) -> CFG_M_REF1_R {
        CFG_M_REF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_on_dac0(&self) -> CFG_ON_DAC0_R {
        CFG_ON_DAC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_on_dac1(&self) -> CFG_ON_DAC1_R {
        CFG_ON_DAC1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cfg_sync_a(&self) -> CFG_SYNC_A_R {
        CFG_SYNC_A_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cfg_m_ref0(&mut self) -> CFG_M_REF0_W {
        CFG_M_REF0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cfg_m_ref1(&mut self) -> CFG_M_REF1_W {
        CFG_M_REF1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cfg_on_dac0(&mut self) -> CFG_ON_DAC0_W {
        CFG_ON_DAC0_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cfg_on_dac1(&mut self) -> CFG_ON_DAC1_W {
        CFG_ON_DAC1_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cfg_sync_a(&mut self) -> CFG_SYNC_A_W {
        CFG_SYNC_A_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
