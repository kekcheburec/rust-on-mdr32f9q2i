#[doc = "Register `SEP1_CTRL` reader"]
pub struct R(crate::R<SEP1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEP1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEP1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEP1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEP1_CTRL` writer"]
pub struct W(crate::W<SEP1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEP1_CTRL_SPEC>;
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
impl From<crate::W<SEP1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEP1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN` reader - "]
pub struct EPEN_R(crate::FieldReader<bool, bool>);
impl EPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN` writer - "]
pub struct EPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN_W<'a> {
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
#[doc = "Field `EPRDY` reader - "]
pub struct EPRDY_R(crate::FieldReader<bool, bool>);
impl EPRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPRDY` writer - "]
pub struct EPRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> EPRDY_W<'a> {
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
#[doc = "Field `EPDATASEQ` reader - "]
pub struct EPDATASEQ_R(crate::FieldReader<bool, bool>);
impl EPDATASEQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPDATASEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPDATASEQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPDATASEQ` writer - "]
pub struct EPDATASEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> EPDATASEQ_W<'a> {
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
#[doc = "Field `EPSSTALL` reader - "]
pub struct EPSSTALL_R(crate::FieldReader<bool, bool>);
impl EPSSTALL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPSSTALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPSSTALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPSSTALL` writer - "]
pub struct EPSSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EPSSTALL_W<'a> {
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
#[doc = "Field `EPISOEN` reader - "]
pub struct EPISOEN_R(crate::FieldReader<bool, bool>);
impl EPISOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPISOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPISOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPISOEN` writer - "]
pub struct EPISOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EPISOEN_W<'a> {
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
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eprdy(&self) -> EPRDY_R {
        EPRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epdataseq(&self) -> EPDATASEQ_R {
        EPDATASEQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn epsstall(&self) -> EPSSTALL_R {
        EPSSTALL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn episoen(&self) -> EPISOEN_R {
        EPISOEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn epen(&mut self) -> EPEN_W {
        EPEN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn eprdy(&mut self) -> EPRDY_W {
        EPRDY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn epdataseq(&mut self) -> EPDATASEQ_W {
        EPDATASEQ_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn epsstall(&mut self) -> EPSSTALL_W {
        EPSSTALL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn episoen(&mut self) -> EPISOEN_W {
        EPISOEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_SEP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sep1_ctrl](index.html) module"]
pub struct SEP1_CTRL_SPEC;
impl crate::RegisterSpec for SEP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sep1_ctrl::R](R) reader structure"]
impl crate::Readable for SEP1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sep1_ctrl::W](W) writer structure"]
impl crate::Writable for SEP1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEP1_CTRL to value 0"]
impl crate::Resettable for SEP1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
