#[doc = "Register `RXTX` reader"]
pub struct R(crate::R<RXTX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXTX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXTX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXTX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXTX` writer"]
pub struct W(crate::W<RXTX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXTX_SPEC>;
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
impl From<crate::W<RXTX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXTX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTX_0` reader - "]
pub struct RXTX_0_R(crate::FieldReader<bool, bool>);
impl RXTX_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_0` writer - "]
pub struct RXTX_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_0_W<'a> {
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
#[doc = "Field `RXTX_1` reader - "]
pub struct RXTX_1_R(crate::FieldReader<bool, bool>);
impl RXTX_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_1` writer - "]
pub struct RXTX_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_1_W<'a> {
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
#[doc = "Field `RXTX_2` reader - "]
pub struct RXTX_2_R(crate::FieldReader<bool, bool>);
impl RXTX_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_2` writer - "]
pub struct RXTX_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_2_W<'a> {
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
#[doc = "Field `RXTX_3` reader - "]
pub struct RXTX_3_R(crate::FieldReader<bool, bool>);
impl RXTX_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_3` writer - "]
pub struct RXTX_3_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_3_W<'a> {
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
#[doc = "Field `RXTX_4` reader - "]
pub struct RXTX_4_R(crate::FieldReader<bool, bool>);
impl RXTX_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_4` writer - "]
pub struct RXTX_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_4_W<'a> {
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
#[doc = "Field `RXTX_5` reader - "]
pub struct RXTX_5_R(crate::FieldReader<bool, bool>);
impl RXTX_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_5` writer - "]
pub struct RXTX_5_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_5_W<'a> {
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
#[doc = "Field `RXTX_6` reader - "]
pub struct RXTX_6_R(crate::FieldReader<bool, bool>);
impl RXTX_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_6` writer - "]
pub struct RXTX_6_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_6_W<'a> {
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
#[doc = "Field `RXTX_7` reader - "]
pub struct RXTX_7_R(crate::FieldReader<bool, bool>);
impl RXTX_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_7` writer - "]
pub struct RXTX_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_7_W<'a> {
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
#[doc = "Field `RXTX_8` reader - "]
pub struct RXTX_8_R(crate::FieldReader<bool, bool>);
impl RXTX_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_8` writer - "]
pub struct RXTX_8_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_8_W<'a> {
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
#[doc = "Field `RXTX_9` reader - "]
pub struct RXTX_9_R(crate::FieldReader<bool, bool>);
impl RXTX_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_9` writer - "]
pub struct RXTX_9_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_9_W<'a> {
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
#[doc = "Field `RXTX_10` reader - "]
pub struct RXTX_10_R(crate::FieldReader<bool, bool>);
impl RXTX_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_10` writer - "]
pub struct RXTX_10_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_10_W<'a> {
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
#[doc = "Field `RXTX_11` reader - "]
pub struct RXTX_11_R(crate::FieldReader<bool, bool>);
impl RXTX_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_11` writer - "]
pub struct RXTX_11_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_11_W<'a> {
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
#[doc = "Field `RXTX_12` reader - "]
pub struct RXTX_12_R(crate::FieldReader<bool, bool>);
impl RXTX_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_12` writer - "]
pub struct RXTX_12_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `RXTX_13` reader - "]
pub struct RXTX_13_R(crate::FieldReader<bool, bool>);
impl RXTX_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_13` writer - "]
pub struct RXTX_13_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `RXTX_14` reader - "]
pub struct RXTX_14_R(crate::FieldReader<bool, bool>);
impl RXTX_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_14` writer - "]
pub struct RXTX_14_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `RXTX_15` reader - "]
pub struct RXTX_15_R(crate::FieldReader<bool, bool>);
impl RXTX_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXTX_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXTX_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXTX_15` writer - "]
pub struct RXTX_15_W<'a> {
    w: &'a mut W,
}
impl<'a> RXTX_15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxtx_0(&self) -> RXTX_0_R {
        RXTX_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxtx_1(&self) -> RXTX_1_R {
        RXTX_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxtx_2(&self) -> RXTX_2_R {
        RXTX_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxtx_3(&self) -> RXTX_3_R {
        RXTX_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxtx_4(&self) -> RXTX_4_R {
        RXTX_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxtx_5(&self) -> RXTX_5_R {
        RXTX_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxtx_6(&self) -> RXTX_6_R {
        RXTX_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rxtx_7(&self) -> RXTX_7_R {
        RXTX_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxtx_8(&self) -> RXTX_8_R {
        RXTX_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rxtx_9(&self) -> RXTX_9_R {
        RXTX_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rxtx_10(&self) -> RXTX_10_R {
        RXTX_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rxtx_11(&self) -> RXTX_11_R {
        RXTX_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxtx_12(&self) -> RXTX_12_R {
        RXTX_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rxtx_13(&self) -> RXTX_13_R {
        RXTX_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rxtx_14(&self) -> RXTX_14_R {
        RXTX_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxtx_15(&self) -> RXTX_15_R {
        RXTX_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rxtx_0(&mut self) -> RXTX_0_W {
        RXTX_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rxtx_1(&mut self) -> RXTX_1_W {
        RXTX_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rxtx_2(&mut self) -> RXTX_2_W {
        RXTX_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rxtx_3(&mut self) -> RXTX_3_W {
        RXTX_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxtx_4(&mut self) -> RXTX_4_W {
        RXTX_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxtx_5(&mut self) -> RXTX_5_W {
        RXTX_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rxtx_6(&mut self) -> RXTX_6_W {
        RXTX_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rxtx_7(&mut self) -> RXTX_7_W {
        RXTX_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rxtx_8(&mut self) -> RXTX_8_W {
        RXTX_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rxtx_9(&mut self) -> RXTX_9_W {
        RXTX_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rxtx_10(&mut self) -> RXTX_10_W {
        RXTX_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rxtx_11(&mut self) -> RXTX_11_W {
        RXTX_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rxtx_12(&mut self) -> RXTX_12_W {
        RXTX_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rxtx_13(&mut self) -> RXTX_13_W {
        RXTX_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rxtx_14(&mut self) -> RXTX_14_W {
        RXTX_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rxtx_15(&mut self) -> RXTX_15_W {
        RXTX_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxtx](index.html) module"]
pub struct RXTX_SPEC;
impl crate::RegisterSpec for RXTX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxtx::R](R) reader structure"]
impl crate::Readable for RXTX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxtx::W](W) writer structure"]
impl crate::Writable for RXTX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXTX to value 0"]
impl crate::Resettable for RXTX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
