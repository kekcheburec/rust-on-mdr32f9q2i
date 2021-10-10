#[doc = "Register `ANALOG` reader"]
pub struct R(crate::R<ANALOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANALOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANALOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANALOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANALOG` writer"]
pub struct W(crate::W<ANALOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANALOG_SPEC>;
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
impl From<crate::W<ANALOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANALOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ANALOG_EN_0` reader - "]
pub struct ANALOG_EN_0_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_0` writer - "]
pub struct ANALOG_EN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_0_W<'a> {
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
#[doc = "Field `ANALOG_EN_1` reader - "]
pub struct ANALOG_EN_1_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_1` writer - "]
pub struct ANALOG_EN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_1_W<'a> {
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
#[doc = "Field `ANALOG_EN_2` reader - "]
pub struct ANALOG_EN_2_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_2` writer - "]
pub struct ANALOG_EN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_2_W<'a> {
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
#[doc = "Field `ANALOG_EN_3` reader - "]
pub struct ANALOG_EN_3_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_3` writer - "]
pub struct ANALOG_EN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_3_W<'a> {
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
#[doc = "Field `ANALOG_EN_4` reader - "]
pub struct ANALOG_EN_4_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_4` writer - "]
pub struct ANALOG_EN_4_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_4_W<'a> {
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
#[doc = "Field `ANALOG_EN_5` reader - "]
pub struct ANALOG_EN_5_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_5` writer - "]
pub struct ANALOG_EN_5_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_5_W<'a> {
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
#[doc = "Field `ANALOG_EN_6` reader - "]
pub struct ANALOG_EN_6_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_6` writer - "]
pub struct ANALOG_EN_6_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_6_W<'a> {
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
#[doc = "Field `ANALOG_EN_7` reader - "]
pub struct ANALOG_EN_7_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_7` writer - "]
pub struct ANALOG_EN_7_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_7_W<'a> {
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
#[doc = "Field `ANALOG_EN_8` reader - "]
pub struct ANALOG_EN_8_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_8` writer - "]
pub struct ANALOG_EN_8_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_8_W<'a> {
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
#[doc = "Field `ANALOG_EN_9` reader - "]
pub struct ANALOG_EN_9_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_9` writer - "]
pub struct ANALOG_EN_9_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_9_W<'a> {
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
#[doc = "Field `ANALOG_EN_10` reader - "]
pub struct ANALOG_EN_10_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_10` writer - "]
pub struct ANALOG_EN_10_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_10_W<'a> {
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
#[doc = "Field `ANALOG_EN_11` reader - "]
pub struct ANALOG_EN_11_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_11` writer - "]
pub struct ANALOG_EN_11_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_11_W<'a> {
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
#[doc = "Field `ANALOG_EN_12` reader - "]
pub struct ANALOG_EN_12_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_12` writer - "]
pub struct ANALOG_EN_12_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_12_W<'a> {
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
#[doc = "Field `ANALOG_EN_13` reader - "]
pub struct ANALOG_EN_13_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_13` writer - "]
pub struct ANALOG_EN_13_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_13_W<'a> {
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
#[doc = "Field `ANALOG_EN_14` reader - "]
pub struct ANALOG_EN_14_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_14` writer - "]
pub struct ANALOG_EN_14_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_14_W<'a> {
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
#[doc = "Field `ANALOG_EN_15` reader - "]
pub struct ANALOG_EN_15_R(crate::FieldReader<bool, bool>);
impl ANALOG_EN_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        ANALOG_EN_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ANALOG_EN_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ANALOG_EN_15` writer - "]
pub struct ANALOG_EN_15_W<'a> {
    w: &'a mut W,
}
impl<'a> ANALOG_EN_15_W<'a> {
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
    pub fn analog_en_0(&self) -> ANALOG_EN_0_R {
        ANALOG_EN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn analog_en_1(&self) -> ANALOG_EN_1_R {
        ANALOG_EN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn analog_en_2(&self) -> ANALOG_EN_2_R {
        ANALOG_EN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn analog_en_3(&self) -> ANALOG_EN_3_R {
        ANALOG_EN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn analog_en_4(&self) -> ANALOG_EN_4_R {
        ANALOG_EN_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn analog_en_5(&self) -> ANALOG_EN_5_R {
        ANALOG_EN_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn analog_en_6(&self) -> ANALOG_EN_6_R {
        ANALOG_EN_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn analog_en_7(&self) -> ANALOG_EN_7_R {
        ANALOG_EN_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn analog_en_8(&self) -> ANALOG_EN_8_R {
        ANALOG_EN_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn analog_en_9(&self) -> ANALOG_EN_9_R {
        ANALOG_EN_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn analog_en_10(&self) -> ANALOG_EN_10_R {
        ANALOG_EN_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn analog_en_11(&self) -> ANALOG_EN_11_R {
        ANALOG_EN_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn analog_en_12(&self) -> ANALOG_EN_12_R {
        ANALOG_EN_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn analog_en_13(&self) -> ANALOG_EN_13_R {
        ANALOG_EN_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn analog_en_14(&self) -> ANALOG_EN_14_R {
        ANALOG_EN_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn analog_en_15(&self) -> ANALOG_EN_15_R {
        ANALOG_EN_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn analog_en_0(&mut self) -> ANALOG_EN_0_W {
        ANALOG_EN_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn analog_en_1(&mut self) -> ANALOG_EN_1_W {
        ANALOG_EN_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn analog_en_2(&mut self) -> ANALOG_EN_2_W {
        ANALOG_EN_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn analog_en_3(&mut self) -> ANALOG_EN_3_W {
        ANALOG_EN_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn analog_en_4(&mut self) -> ANALOG_EN_4_W {
        ANALOG_EN_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn analog_en_5(&mut self) -> ANALOG_EN_5_W {
        ANALOG_EN_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn analog_en_6(&mut self) -> ANALOG_EN_6_W {
        ANALOG_EN_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn analog_en_7(&mut self) -> ANALOG_EN_7_W {
        ANALOG_EN_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn analog_en_8(&mut self) -> ANALOG_EN_8_W {
        ANALOG_EN_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn analog_en_9(&mut self) -> ANALOG_EN_9_W {
        ANALOG_EN_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn analog_en_10(&mut self) -> ANALOG_EN_10_W {
        ANALOG_EN_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn analog_en_11(&mut self) -> ANALOG_EN_11_W {
        ANALOG_EN_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn analog_en_12(&mut self) -> ANALOG_EN_12_W {
        ANALOG_EN_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn analog_en_13(&mut self) -> ANALOG_EN_13_W {
        ANALOG_EN_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn analog_en_14(&mut self) -> ANALOG_EN_14_W {
        ANALOG_EN_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn analog_en_15(&mut self) -> ANALOG_EN_15_W {
        ANALOG_EN_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Analog Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [analog](index.html) module"]
pub struct ANALOG_SPEC;
impl crate::RegisterSpec for ANALOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [analog::R](R) reader structure"]
impl crate::Readable for ANALOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [analog::W](W) writer structure"]
impl crate::Writable for ANALOG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANALOG to value 0"]
impl crate::Resettable for ANALOG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
