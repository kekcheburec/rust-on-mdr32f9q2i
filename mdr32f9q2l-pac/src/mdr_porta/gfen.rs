#[doc = "Register `GFEN` reader"]
pub struct R(crate::R<GFEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GFEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GFEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GFEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GFEN` writer"]
pub struct W(crate::W<GFEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GFEN_SPEC>;
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
impl From<crate::W<GFEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GFEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GFEN_0` reader - "]
pub struct GFEN_0_R(crate::FieldReader<bool, bool>);
impl GFEN_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_0` writer - "]
pub struct GFEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_0_W<'a> {
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
#[doc = "Field `GFEN_1` reader - "]
pub struct GFEN_1_R(crate::FieldReader<bool, bool>);
impl GFEN_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_1` writer - "]
pub struct GFEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_1_W<'a> {
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
#[doc = "Field `GFEN_2` reader - "]
pub struct GFEN_2_R(crate::FieldReader<bool, bool>);
impl GFEN_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_2` writer - "]
pub struct GFEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_2_W<'a> {
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
#[doc = "Field `GFEN_3` reader - "]
pub struct GFEN_3_R(crate::FieldReader<bool, bool>);
impl GFEN_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_3` writer - "]
pub struct GFEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_3_W<'a> {
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
#[doc = "Field `GFEN_4` reader - "]
pub struct GFEN_4_R(crate::FieldReader<bool, bool>);
impl GFEN_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_4` writer - "]
pub struct GFEN_4_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_4_W<'a> {
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
#[doc = "Field `GFEN_5` reader - "]
pub struct GFEN_5_R(crate::FieldReader<bool, bool>);
impl GFEN_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_5` writer - "]
pub struct GFEN_5_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_5_W<'a> {
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
#[doc = "Field `GFEN_6` reader - "]
pub struct GFEN_6_R(crate::FieldReader<bool, bool>);
impl GFEN_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_6` writer - "]
pub struct GFEN_6_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_6_W<'a> {
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
#[doc = "Field `GFEN_7` reader - "]
pub struct GFEN_7_R(crate::FieldReader<bool, bool>);
impl GFEN_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_7` writer - "]
pub struct GFEN_7_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_7_W<'a> {
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
#[doc = "Field `GFEN_8` reader - "]
pub struct GFEN_8_R(crate::FieldReader<bool, bool>);
impl GFEN_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_8` writer - "]
pub struct GFEN_8_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_8_W<'a> {
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
#[doc = "Field `GFEN_9` reader - "]
pub struct GFEN_9_R(crate::FieldReader<bool, bool>);
impl GFEN_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_9` writer - "]
pub struct GFEN_9_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_9_W<'a> {
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
#[doc = "Field `GFEN_10` reader - "]
pub struct GFEN_10_R(crate::FieldReader<bool, bool>);
impl GFEN_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_10` writer - "]
pub struct GFEN_10_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_10_W<'a> {
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
#[doc = "Field `GFEN_11` reader - "]
pub struct GFEN_11_R(crate::FieldReader<bool, bool>);
impl GFEN_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_11` writer - "]
pub struct GFEN_11_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_11_W<'a> {
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
#[doc = "Field `GFEN_12` reader - "]
pub struct GFEN_12_R(crate::FieldReader<bool, bool>);
impl GFEN_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_12` writer - "]
pub struct GFEN_12_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_12_W<'a> {
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
#[doc = "Field `GFEN_13` reader - "]
pub struct GFEN_13_R(crate::FieldReader<bool, bool>);
impl GFEN_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_13` writer - "]
pub struct GFEN_13_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_13_W<'a> {
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
#[doc = "Field `GFEN_14` reader - "]
pub struct GFEN_14_R(crate::FieldReader<bool, bool>);
impl GFEN_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_14` writer - "]
pub struct GFEN_14_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_14_W<'a> {
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
#[doc = "Field `GFEN_15` reader - "]
pub struct GFEN_15_R(crate::FieldReader<bool, bool>);
impl GFEN_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        GFEN_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GFEN_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GFEN_15` writer - "]
pub struct GFEN_15_W<'a> {
    w: &'a mut W,
}
impl<'a> GFEN_15_W<'a> {
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
    pub fn gfen_0(&self) -> GFEN_0_R {
        GFEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gfen_1(&self) -> GFEN_1_R {
        GFEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gfen_2(&self) -> GFEN_2_R {
        GFEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gfen_3(&self) -> GFEN_3_R {
        GFEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gfen_4(&self) -> GFEN_4_R {
        GFEN_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gfen_5(&self) -> GFEN_5_R {
        GFEN_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gfen_6(&self) -> GFEN_6_R {
        GFEN_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gfen_7(&self) -> GFEN_7_R {
        GFEN_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gfen_8(&self) -> GFEN_8_R {
        GFEN_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gfen_9(&self) -> GFEN_9_R {
        GFEN_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gfen_10(&self) -> GFEN_10_R {
        GFEN_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gfen_11(&self) -> GFEN_11_R {
        GFEN_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gfen_12(&self) -> GFEN_12_R {
        GFEN_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gfen_13(&self) -> GFEN_13_R {
        GFEN_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gfen_14(&self) -> GFEN_14_R {
        GFEN_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gfen_15(&self) -> GFEN_15_R {
        GFEN_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gfen_0(&mut self) -> GFEN_0_W {
        GFEN_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gfen_1(&mut self) -> GFEN_1_W {
        GFEN_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gfen_2(&mut self) -> GFEN_2_W {
        GFEN_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gfen_3(&mut self) -> GFEN_3_W {
        GFEN_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gfen_4(&mut self) -> GFEN_4_W {
        GFEN_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gfen_5(&mut self) -> GFEN_5_W {
        GFEN_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gfen_6(&mut self) -> GFEN_6_W {
        GFEN_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gfen_7(&mut self) -> GFEN_7_W {
        GFEN_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gfen_8(&mut self) -> GFEN_8_W {
        GFEN_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gfen_9(&mut self) -> GFEN_9_W {
        GFEN_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gfen_10(&mut self) -> GFEN_10_W {
        GFEN_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gfen_11(&mut self) -> GFEN_11_W {
        GFEN_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gfen_12(&mut self) -> GFEN_12_W {
        GFEN_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gfen_13(&mut self) -> GFEN_13_W {
        GFEN_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gfen_14(&mut self) -> GFEN_14_W {
        GFEN_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gfen_15(&mut self) -> GFEN_15_W {
        GFEN_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Filter Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gfen](index.html) module"]
pub struct GFEN_SPEC;
impl crate::RegisterSpec for GFEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gfen::R](R) reader structure"]
impl crate::Readable for GFEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gfen::W](W) writer structure"]
impl crate::Writable for GFEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GFEN to value 0"]
impl crate::Resettable for GFEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
