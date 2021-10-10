#[doc = "Register `OE` reader"]
pub struct R(crate::R<OE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OE` writer"]
pub struct W(crate::W<OE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OE_SPEC>;
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
impl From<crate::W<OE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OE_0` reader - "]
pub struct OE_0_R(crate::FieldReader<bool, bool>);
impl OE_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_0` writer - "]
pub struct OE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_0_W<'a> {
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
#[doc = "Field `OE_1` reader - "]
pub struct OE_1_R(crate::FieldReader<bool, bool>);
impl OE_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_1` writer - "]
pub struct OE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_1_W<'a> {
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
#[doc = "Field `OE_2` reader - "]
pub struct OE_2_R(crate::FieldReader<bool, bool>);
impl OE_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_2` writer - "]
pub struct OE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_2_W<'a> {
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
#[doc = "Field `OE_3` reader - "]
pub struct OE_3_R(crate::FieldReader<bool, bool>);
impl OE_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_3` writer - "]
pub struct OE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_3_W<'a> {
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
#[doc = "Field `OE_4` reader - "]
pub struct OE_4_R(crate::FieldReader<bool, bool>);
impl OE_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_4` writer - "]
pub struct OE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_4_W<'a> {
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
#[doc = "Field `OE_5` reader - "]
pub struct OE_5_R(crate::FieldReader<bool, bool>);
impl OE_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_5` writer - "]
pub struct OE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_5_W<'a> {
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
#[doc = "Field `OE_6` reader - "]
pub struct OE_6_R(crate::FieldReader<bool, bool>);
impl OE_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_6` writer - "]
pub struct OE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_6_W<'a> {
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
#[doc = "Field `OE_7` reader - "]
pub struct OE_7_R(crate::FieldReader<bool, bool>);
impl OE_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_7` writer - "]
pub struct OE_7_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_7_W<'a> {
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
#[doc = "Field `OE_8` reader - "]
pub struct OE_8_R(crate::FieldReader<bool, bool>);
impl OE_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_8` writer - "]
pub struct OE_8_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_8_W<'a> {
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
#[doc = "Field `OE_9` reader - "]
pub struct OE_9_R(crate::FieldReader<bool, bool>);
impl OE_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_9` writer - "]
pub struct OE_9_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_9_W<'a> {
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
#[doc = "Field `OE_10` reader - "]
pub struct OE_10_R(crate::FieldReader<bool, bool>);
impl OE_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_10` writer - "]
pub struct OE_10_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_10_W<'a> {
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
#[doc = "Field `OE_11` reader - "]
pub struct OE_11_R(crate::FieldReader<bool, bool>);
impl OE_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_11` writer - "]
pub struct OE_11_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_11_W<'a> {
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
#[doc = "Field `OE_12` reader - "]
pub struct OE_12_R(crate::FieldReader<bool, bool>);
impl OE_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_12` writer - "]
pub struct OE_12_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_12_W<'a> {
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
#[doc = "Field `OE_13` reader - "]
pub struct OE_13_R(crate::FieldReader<bool, bool>);
impl OE_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_13` writer - "]
pub struct OE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_13_W<'a> {
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
#[doc = "Field `OE_14` reader - "]
pub struct OE_14_R(crate::FieldReader<bool, bool>);
impl OE_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_14` writer - "]
pub struct OE_14_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_14_W<'a> {
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
#[doc = "Field `OE_15` reader - "]
pub struct OE_15_R(crate::FieldReader<bool, bool>);
impl OE_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        OE_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OE_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OE_15` writer - "]
pub struct OE_15_W<'a> {
    w: &'a mut W,
}
impl<'a> OE_15_W<'a> {
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
    pub fn oe_0(&self) -> OE_0_R {
        OE_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oe_1(&self) -> OE_1_R {
        OE_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oe_2(&self) -> OE_2_R {
        OE_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oe_3(&self) -> OE_3_R {
        OE_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn oe_4(&self) -> OE_4_R {
        OE_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn oe_5(&self) -> OE_5_R {
        OE_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn oe_6(&self) -> OE_6_R {
        OE_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oe_7(&self) -> OE_7_R {
        OE_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn oe_8(&self) -> OE_8_R {
        OE_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oe_9(&self) -> OE_9_R {
        OE_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oe_10(&self) -> OE_10_R {
        OE_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oe_11(&self) -> OE_11_R {
        OE_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oe_12(&self) -> OE_12_R {
        OE_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn oe_13(&self) -> OE_13_R {
        OE_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn oe_14(&self) -> OE_14_R {
        OE_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oe_15(&self) -> OE_15_R {
        OE_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn oe_0(&mut self) -> OE_0_W {
        OE_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn oe_1(&mut self) -> OE_1_W {
        OE_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn oe_2(&mut self) -> OE_2_W {
        OE_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn oe_3(&mut self) -> OE_3_W {
        OE_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn oe_4(&mut self) -> OE_4_W {
        OE_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn oe_5(&mut self) -> OE_5_W {
        OE_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn oe_6(&mut self) -> OE_6_W {
        OE_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn oe_7(&mut self) -> OE_7_W {
        OE_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn oe_8(&mut self) -> OE_8_W {
        OE_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn oe_9(&mut self) -> OE_9_W {
        OE_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn oe_10(&mut self) -> OE_10_W {
        OE_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn oe_11(&mut self) -> OE_11_W {
        OE_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn oe_12(&mut self) -> OE_12_W {
        OE_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn oe_13(&mut self) -> OE_13_W {
        OE_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn oe_14(&mut self) -> OE_14_W {
        OE_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn oe_15(&mut self) -> OE_15_W {
        OE_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Output Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oe](index.html) module"]
pub struct OE_SPEC;
impl crate::RegisterSpec for OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oe::R](R) reader structure"]
impl crate::Readable for OE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oe::W](W) writer structure"]
impl crate::Writable for OE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OE to value 0"]
impl crate::Resettable for OE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
