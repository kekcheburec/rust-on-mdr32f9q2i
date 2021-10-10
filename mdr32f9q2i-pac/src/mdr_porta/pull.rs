#[doc = "Register `PULL` reader"]
pub struct R(crate::R<PULL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULL` writer"]
pub struct W(crate::W<PULL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULL_SPEC>;
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
impl From<crate::W<PULL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULL_DOWN_0` reader - "]
pub struct PULL_DOWN_0_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_0` writer - "]
pub struct PULL_DOWN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_0_W<'a> {
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
#[doc = "Field `PULL_DOWN_1` reader - "]
pub struct PULL_DOWN_1_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_1` writer - "]
pub struct PULL_DOWN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_1_W<'a> {
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
#[doc = "Field `PULL_DOWN_2` reader - "]
pub struct PULL_DOWN_2_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_2` writer - "]
pub struct PULL_DOWN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_2_W<'a> {
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
#[doc = "Field `PULL_DOWN_3` reader - "]
pub struct PULL_DOWN_3_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_3` writer - "]
pub struct PULL_DOWN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_3_W<'a> {
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
#[doc = "Field `PULL_DOWN_4` reader - "]
pub struct PULL_DOWN_4_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_4` writer - "]
pub struct PULL_DOWN_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_4_W<'a> {
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
#[doc = "Field `PULL_DOWN_5` reader - "]
pub struct PULL_DOWN_5_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_5` writer - "]
pub struct PULL_DOWN_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_5_W<'a> {
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
#[doc = "Field `PULL_DOWN_6` reader - "]
pub struct PULL_DOWN_6_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_6` writer - "]
pub struct PULL_DOWN_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_6_W<'a> {
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
#[doc = "Field `PULL_DOWN_7` reader - "]
pub struct PULL_DOWN_7_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_7` writer - "]
pub struct PULL_DOWN_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_7_W<'a> {
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
#[doc = "Field `PULL_DOWN_8` reader - "]
pub struct PULL_DOWN_8_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_8` writer - "]
pub struct PULL_DOWN_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_8_W<'a> {
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
#[doc = "Field `PULL_DOWN_9` reader - "]
pub struct PULL_DOWN_9_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_9` writer - "]
pub struct PULL_DOWN_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_9_W<'a> {
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
#[doc = "Field `PULL_DOWN_10` reader - "]
pub struct PULL_DOWN_10_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_10` writer - "]
pub struct PULL_DOWN_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_10_W<'a> {
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
#[doc = "Field `PULL_DOWN_11` reader - "]
pub struct PULL_DOWN_11_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_11` writer - "]
pub struct PULL_DOWN_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_11_W<'a> {
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
#[doc = "Field `PULL_DOWN_12` reader - "]
pub struct PULL_DOWN_12_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_12` writer - "]
pub struct PULL_DOWN_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_12_W<'a> {
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
#[doc = "Field `PULL_DOWN_13` reader - "]
pub struct PULL_DOWN_13_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_13` writer - "]
pub struct PULL_DOWN_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_13_W<'a> {
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
#[doc = "Field `PULL_DOWN_14` reader - "]
pub struct PULL_DOWN_14_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_14` writer - "]
pub struct PULL_DOWN_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_14_W<'a> {
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
#[doc = "Field `PULL_DOWN_15` reader - "]
pub struct PULL_DOWN_15_R(crate::FieldReader<bool, bool>);
impl PULL_DOWN_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_DOWN_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_DOWN_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_DOWN_15` writer - "]
pub struct PULL_DOWN_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_DOWN_15_W<'a> {
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
#[doc = "Field `PULL_UP_0` reader - "]
pub struct PULL_UP_0_R(crate::FieldReader<bool, bool>);
impl PULL_UP_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_0` writer - "]
pub struct PULL_UP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PULL_UP_1` reader - "]
pub struct PULL_UP_1_R(crate::FieldReader<bool, bool>);
impl PULL_UP_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_1` writer - "]
pub struct PULL_UP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PULL_UP_2` reader - "]
pub struct PULL_UP_2_R(crate::FieldReader<bool, bool>);
impl PULL_UP_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_2` writer - "]
pub struct PULL_UP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PULL_UP_3` reader - "]
pub struct PULL_UP_3_R(crate::FieldReader<bool, bool>);
impl PULL_UP_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_3` writer - "]
pub struct PULL_UP_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PULL_UP_4` reader - "]
pub struct PULL_UP_4_R(crate::FieldReader<bool, bool>);
impl PULL_UP_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_4` writer - "]
pub struct PULL_UP_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PULL_UP_5` reader - "]
pub struct PULL_UP_5_R(crate::FieldReader<bool, bool>);
impl PULL_UP_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_5` writer - "]
pub struct PULL_UP_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PULL_UP_6` reader - "]
pub struct PULL_UP_6_R(crate::FieldReader<bool, bool>);
impl PULL_UP_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_6` writer - "]
pub struct PULL_UP_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PULL_UP_7` reader - "]
pub struct PULL_UP_7_R(crate::FieldReader<bool, bool>);
impl PULL_UP_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_7` writer - "]
pub struct PULL_UP_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PULL_UP_8` reader - "]
pub struct PULL_UP_8_R(crate::FieldReader<bool, bool>);
impl PULL_UP_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_8` writer - "]
pub struct PULL_UP_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PULL_UP_9` reader - "]
pub struct PULL_UP_9_R(crate::FieldReader<bool, bool>);
impl PULL_UP_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_9` writer - "]
pub struct PULL_UP_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PULL_UP_10` reader - "]
pub struct PULL_UP_10_R(crate::FieldReader<bool, bool>);
impl PULL_UP_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_10` writer - "]
pub struct PULL_UP_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PULL_UP_11` reader - "]
pub struct PULL_UP_11_R(crate::FieldReader<bool, bool>);
impl PULL_UP_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_11` writer - "]
pub struct PULL_UP_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PULL_UP_12` reader - "]
pub struct PULL_UP_12_R(crate::FieldReader<bool, bool>);
impl PULL_UP_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_12` writer - "]
pub struct PULL_UP_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_12_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PULL_UP_13` reader - "]
pub struct PULL_UP_13_R(crate::FieldReader<bool, bool>);
impl PULL_UP_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_13` writer - "]
pub struct PULL_UP_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PULL_UP_14` reader - "]
pub struct PULL_UP_14_R(crate::FieldReader<bool, bool>);
impl PULL_UP_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_14` writer - "]
pub struct PULL_UP_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PULL_UP_15` reader - "]
pub struct PULL_UP_15_R(crate::FieldReader<bool, bool>);
impl PULL_UP_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULL_UP_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_UP_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_UP_15` writer - "]
pub struct PULL_UP_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_UP_15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pull_down_0(&self) -> PULL_DOWN_0_R {
        PULL_DOWN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pull_down_1(&self) -> PULL_DOWN_1_R {
        PULL_DOWN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pull_down_2(&self) -> PULL_DOWN_2_R {
        PULL_DOWN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pull_down_3(&self) -> PULL_DOWN_3_R {
        PULL_DOWN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pull_down_4(&self) -> PULL_DOWN_4_R {
        PULL_DOWN_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pull_down_5(&self) -> PULL_DOWN_5_R {
        PULL_DOWN_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pull_down_6(&self) -> PULL_DOWN_6_R {
        PULL_DOWN_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_down_7(&self) -> PULL_DOWN_7_R {
        PULL_DOWN_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_down_8(&self) -> PULL_DOWN_8_R {
        PULL_DOWN_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pull_down_9(&self) -> PULL_DOWN_9_R {
        PULL_DOWN_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pull_down_10(&self) -> PULL_DOWN_10_R {
        PULL_DOWN_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pull_down_11(&self) -> PULL_DOWN_11_R {
        PULL_DOWN_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pull_down_12(&self) -> PULL_DOWN_12_R {
        PULL_DOWN_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pull_down_13(&self) -> PULL_DOWN_13_R {
        PULL_DOWN_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pull_down_14(&self) -> PULL_DOWN_14_R {
        PULL_DOWN_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pull_down_15(&self) -> PULL_DOWN_15_R {
        PULL_DOWN_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pull_up_0(&self) -> PULL_UP_0_R {
        PULL_UP_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pull_up_1(&self) -> PULL_UP_1_R {
        PULL_UP_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pull_up_2(&self) -> PULL_UP_2_R {
        PULL_UP_2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pull_up_3(&self) -> PULL_UP_3_R {
        PULL_UP_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pull_up_4(&self) -> PULL_UP_4_R {
        PULL_UP_4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pull_up_5(&self) -> PULL_UP_5_R {
        PULL_UP_5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pull_up_6(&self) -> PULL_UP_6_R {
        PULL_UP_6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pull_up_7(&self) -> PULL_UP_7_R {
        PULL_UP_7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pull_up_8(&self) -> PULL_UP_8_R {
        PULL_UP_8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pull_up_9(&self) -> PULL_UP_9_R {
        PULL_UP_9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pull_up_10(&self) -> PULL_UP_10_R {
        PULL_UP_10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pull_up_11(&self) -> PULL_UP_11_R {
        PULL_UP_11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pull_up_12(&self) -> PULL_UP_12_R {
        PULL_UP_12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pull_up_13(&self) -> PULL_UP_13_R {
        PULL_UP_13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pull_up_14(&self) -> PULL_UP_14_R {
        PULL_UP_14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pull_up_15(&self) -> PULL_UP_15_R {
        PULL_UP_15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pull_down_0(&mut self) -> PULL_DOWN_0_W {
        PULL_DOWN_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pull_down_1(&mut self) -> PULL_DOWN_1_W {
        PULL_DOWN_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pull_down_2(&mut self) -> PULL_DOWN_2_W {
        PULL_DOWN_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pull_down_3(&mut self) -> PULL_DOWN_3_W {
        PULL_DOWN_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pull_down_4(&mut self) -> PULL_DOWN_4_W {
        PULL_DOWN_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pull_down_5(&mut self) -> PULL_DOWN_5_W {
        PULL_DOWN_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pull_down_6(&mut self) -> PULL_DOWN_6_W {
        PULL_DOWN_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pull_down_7(&mut self) -> PULL_DOWN_7_W {
        PULL_DOWN_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pull_down_8(&mut self) -> PULL_DOWN_8_W {
        PULL_DOWN_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pull_down_9(&mut self) -> PULL_DOWN_9_W {
        PULL_DOWN_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pull_down_10(&mut self) -> PULL_DOWN_10_W {
        PULL_DOWN_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pull_down_11(&mut self) -> PULL_DOWN_11_W {
        PULL_DOWN_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pull_down_12(&mut self) -> PULL_DOWN_12_W {
        PULL_DOWN_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pull_down_13(&mut self) -> PULL_DOWN_13_W {
        PULL_DOWN_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pull_down_14(&mut self) -> PULL_DOWN_14_W {
        PULL_DOWN_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pull_down_15(&mut self) -> PULL_DOWN_15_W {
        PULL_DOWN_15_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pull_up_0(&mut self) -> PULL_UP_0_W {
        PULL_UP_0_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pull_up_1(&mut self) -> PULL_UP_1_W {
        PULL_UP_1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pull_up_2(&mut self) -> PULL_UP_2_W {
        PULL_UP_2_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pull_up_3(&mut self) -> PULL_UP_3_W {
        PULL_UP_3_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pull_up_4(&mut self) -> PULL_UP_4_W {
        PULL_UP_4_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pull_up_5(&mut self) -> PULL_UP_5_W {
        PULL_UP_5_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pull_up_6(&mut self) -> PULL_UP_6_W {
        PULL_UP_6_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pull_up_7(&mut self) -> PULL_UP_7_W {
        PULL_UP_7_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pull_up_8(&mut self) -> PULL_UP_8_W {
        PULL_UP_8_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pull_up_9(&mut self) -> PULL_UP_9_W {
        PULL_UP_9_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn pull_up_10(&mut self) -> PULL_UP_10_W {
        PULL_UP_10_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pull_up_11(&mut self) -> PULL_UP_11_W {
        PULL_UP_11_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pull_up_12(&mut self) -> PULL_UP_12_W {
        PULL_UP_12_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pull_up_13(&mut self) -> PULL_UP_13_W {
        PULL_UP_13_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pull_up_14(&mut self) -> PULL_UP_14_W {
        PULL_UP_14_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pull_up_15(&mut self) -> PULL_UP_15_W {
        PULL_UP_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Pull Up/Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pull](index.html) module"]
pub struct PULL_SPEC;
impl crate::RegisterSpec for PULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pull::R](R) reader structure"]
impl crate::Readable for PULL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pull::W](W) writer structure"]
impl crate::Writable for PULL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULL to value 0"]
impl crate::Resettable for PULL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
