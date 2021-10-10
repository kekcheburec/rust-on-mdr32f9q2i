#[doc = "Register `PVDCS` reader"]
pub struct R(crate::R<PVDCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PVDCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PVDCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PVDCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PVDCS` writer"]
pub struct W(crate::W<PVDCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PVDCS_SPEC>;
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
impl From<crate::W<PVDCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PVDCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVDEN` reader - "]
pub struct PVDEN_R(crate::FieldReader<bool, bool>);
impl PVDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVDEN` writer - "]
pub struct PVDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PVDEN_W<'a> {
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
#[doc = "Field `PBLS` reader - "]
pub struct PBLS_R(crate::FieldReader<u8, u8>);
impl PBLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PBLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBLS` writer - "]
pub struct PBLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PBLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `PLS` reader - "]
pub struct PLS_R(crate::FieldReader<u8, u8>);
impl PLS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLS` writer - "]
pub struct PLS_W<'a> {
    w: &'a mut W,
}
impl<'a> PLS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `PVBD` reader - "]
pub struct PVBD_R(crate::FieldReader<bool, bool>);
impl PVBD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVBD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVBD` writer - "]
pub struct PVBD_W<'a> {
    w: &'a mut W,
}
impl<'a> PVBD_W<'a> {
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
#[doc = "Field `PVD` reader - "]
pub struct PVD_R(crate::FieldReader<bool, bool>);
impl PVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PVD` writer - "]
pub struct PVD_W<'a> {
    w: &'a mut W,
}
impl<'a> PVD_W<'a> {
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
#[doc = "Field `IEPVBD` reader - "]
pub struct IEPVBD_R(crate::FieldReader<bool, bool>);
impl IEPVBD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPVBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPVBD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEPVBD` writer - "]
pub struct IEPVBD_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPVBD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `IEPVD` reader - "]
pub struct IEPVD_R(crate::FieldReader<bool, bool>);
impl IEPVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        IEPVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IEPVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IEPVD` writer - "]
pub struct IEPVD_W<'a> {
    w: &'a mut W,
}
impl<'a> IEPVD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `INVB` reader - "]
pub struct INVB_R(crate::FieldReader<bool, bool>);
impl INVB_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVB` writer - "]
pub struct INVB_W<'a> {
    w: &'a mut W,
}
impl<'a> INVB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `INV` reader - "]
pub struct INV_R(crate::FieldReader<bool, bool>);
impl INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INV` writer - "]
pub struct INV_W<'a> {
    w: &'a mut W,
}
impl<'a> INV_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pvden(&self) -> PVDEN_R {
        PVDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn pbls(&self) -> PBLS_R {
        PBLS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pvbd(&self) -> PVBD_R {
        PVBD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pvd(&self) -> PVD_R {
        PVD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn iepvbd(&self) -> IEPVBD_R {
        IEPVBD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn iepvd(&self) -> IEPVD_R {
        IEPVD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn invb(&self) -> INVB_R {
        INVB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pvden(&mut self) -> PVDEN_W {
        PVDEN_W { w: self }
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn pbls(&mut self) -> PBLS_W {
        PBLS_W { w: self }
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn pls(&mut self) -> PLS_W {
        PLS_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pvbd(&mut self) -> PVBD_W {
        PVBD_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pvd(&mut self) -> PVD_W {
        PVD_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn iepvbd(&mut self) -> IEPVBD_W {
        IEPVBD_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn iepvd(&mut self) -> IEPVD_W {
        IEPVD_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn invb(&mut self) -> INVB_W {
        INVB_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W {
        INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "POWER Power Detector Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pvdcs](index.html) module"]
pub struct PVDCS_SPEC;
impl crate::RegisterSpec for PVDCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pvdcs::R](R) reader structure"]
impl crate::Readable for PVDCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pvdcs::W](W) writer structure"]
impl crate::Writable for PVDCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PVDCS to value 0"]
impl crate::Resettable for PVDCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
