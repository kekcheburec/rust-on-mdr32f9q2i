#[doc = "Register `CTR` reader"]
pub struct R(crate::R<CTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTR` writer"]
pub struct W(crate::W<CTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTR_SPEC>;
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
impl From<crate::W<CTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `S_I2C` reader - "]
pub struct S_I2C_R(crate::FieldReader<bool, bool>);
impl S_I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        S_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for S_I2C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `S_I2C` writer - "]
pub struct S_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> S_I2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `EN_INT` reader - "]
pub struct EN_INT_R(crate::FieldReader<bool, bool>);
impl EN_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_INT` writer - "]
pub struct EN_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `EN_I2C` reader - "]
pub struct EN_I2C_R(crate::FieldReader<bool, bool>);
impl EN_I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_I2C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_I2C` writer - "]
pub struct EN_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_I2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn s_i2c(&self) -> S_I2C_R {
        S_I2C_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn en_int(&self) -> EN_INT_R {
        EN_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn en_i2c(&self) -> EN_I2C_R {
        EN_I2C_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn s_i2c(&mut self) -> S_I2C_W {
        S_I2C_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn en_int(&mut self) -> EN_INT_W {
        EN_INT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn en_i2c(&mut self) -> EN_I2C_W {
        EN_I2C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](index.html) module"]
pub struct CTR_SPEC;
impl crate::RegisterSpec for CTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctr::R](R) reader structure"]
impl crate::Readable for CTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctr::W](W) writer structure"]
impl crate::Writable for CTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTR to value 0"]
impl crate::Resettable for CTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
