#[doc = "Register `REG_0E` reader"]
pub struct R(crate::R<REG_0E_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_0E_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_0E_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_0E_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_0E` writer"]
pub struct W(crate::W<REG_0E_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_0E_SPEC>;
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
impl From<crate::W<REG_0E_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_0E_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - "]
pub struct LOW_R(crate::FieldReader<u8, u8>);
impl LOW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOW` writer - "]
pub struct LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SelectRI` reader - "]
pub struct SELECTRI_R(crate::FieldReader<u8, u8>);
impl SELECTRI_R {
    pub(crate) fn new(bits: u8) -> Self {
        SELECTRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SELECTRI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SelectRI` writer - "]
pub struct SELECTRI_W<'a> {
    w: &'a mut W,
}
impl<'a> SELECTRI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `JTAGA` reader - "]
pub struct JTAGA_R(crate::FieldReader<bool, bool>);
impl JTAGA_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAGA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAGA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAGA` writer - "]
pub struct JTAGA_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAGA_W<'a> {
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
#[doc = "Field `JTAGB` reader - "]
pub struct JTAGB_R(crate::FieldReader<bool, bool>);
impl JTAGB_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAGB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAGB` writer - "]
pub struct JTAGB_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAGB_W<'a> {
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
#[doc = "Field `Trim` reader - "]
pub struct TRIM_R(crate::FieldReader<u8, u8>);
impl TRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Trim` writer - "]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `FPOR` reader - "]
pub struct FPOR_R(crate::FieldReader<bool, bool>);
impl FPOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPOR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPOR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPOR` writer - "]
pub struct FPOR_W<'a> {
    w: &'a mut W,
}
impl<'a> FPOR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `BKP_REG` reader - "]
pub struct BKP_REG_R(crate::FieldReader<u32, u32>);
impl BKP_REG_R {
    pub(crate) fn new(bits: u32) -> Self {
        BKP_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_REG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP_REG` writer - "]
pub struct BKP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 12)) | ((value as u32 & 0x000f_ffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn select_ri(&self) -> SELECTRI_R {
        SELECTRI_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jtaga(&self) -> JTAGA_R {
        JTAGA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jtagb(&self) -> JTAGB_R {
        JTAGB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fpor(&self) -> FPOR_R {
        FPOR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new(((self.bits >> 12) & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn low(&mut self) -> LOW_W {
        LOW_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn select_ri(&mut self) -> SELECTRI_W {
        SELECTRI_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn jtaga(&mut self) -> JTAGA_W {
        JTAGA_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn jtagb(&mut self) -> JTAGB_W {
        JTAGB_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn fpor(&mut self) -> FPOR_W {
        FPOR_W { w: self }
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn bkp_reg(&mut self) -> BKP_REG_W {
        BKP_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_0e](index.html) module"]
pub struct REG_0E_SPEC;
impl crate::RegisterSpec for REG_0E_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_0e::R](R) reader structure"]
impl crate::Readable for REG_0E_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_0e::W](W) writer structure"]
impl crate::Writable for REG_0E_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_0E to value 0"]
impl crate::Resettable for REG_0E_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
