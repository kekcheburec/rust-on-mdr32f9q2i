#[doc = "Register `PD` reader"]
pub struct R(crate::R<PD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PD` writer"]
pub struct W(crate::W<PD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PD_SPEC>;
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
impl From<crate::W<PD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD_0` reader - "]
pub struct PD_0_R(crate::FieldReader<bool, bool>);
impl PD_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_0` writer - "]
pub struct PD_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_0_W<'a> {
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
#[doc = "Field `PD_1` reader - "]
pub struct PD_1_R(crate::FieldReader<bool, bool>);
impl PD_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_1` writer - "]
pub struct PD_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_1_W<'a> {
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
#[doc = "Field `PD_2` reader - "]
pub struct PD_2_R(crate::FieldReader<bool, bool>);
impl PD_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_2` writer - "]
pub struct PD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_2_W<'a> {
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
#[doc = "Field `PD_3` reader - "]
pub struct PD_3_R(crate::FieldReader<bool, bool>);
impl PD_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_3` writer - "]
pub struct PD_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_3_W<'a> {
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
#[doc = "Field `PD_4` reader - "]
pub struct PD_4_R(crate::FieldReader<bool, bool>);
impl PD_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_4` writer - "]
pub struct PD_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_4_W<'a> {
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
#[doc = "Field `PD_5` reader - "]
pub struct PD_5_R(crate::FieldReader<bool, bool>);
impl PD_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_5` writer - "]
pub struct PD_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_5_W<'a> {
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
#[doc = "Field `PD_6` reader - "]
pub struct PD_6_R(crate::FieldReader<bool, bool>);
impl PD_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_6` writer - "]
pub struct PD_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_6_W<'a> {
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
#[doc = "Field `PD_7` reader - "]
pub struct PD_7_R(crate::FieldReader<bool, bool>);
impl PD_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_7` writer - "]
pub struct PD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_7_W<'a> {
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
#[doc = "Field `PD_8` reader - "]
pub struct PD_8_R(crate::FieldReader<bool, bool>);
impl PD_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_8` writer - "]
pub struct PD_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_8_W<'a> {
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
#[doc = "Field `PD_9` reader - "]
pub struct PD_9_R(crate::FieldReader<bool, bool>);
impl PD_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_9` writer - "]
pub struct PD_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_9_W<'a> {
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
#[doc = "Field `PD_10` reader - "]
pub struct PD_10_R(crate::FieldReader<bool, bool>);
impl PD_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_10` writer - "]
pub struct PD_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_10_W<'a> {
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
#[doc = "Field `PD_11` reader - "]
pub struct PD_11_R(crate::FieldReader<bool, bool>);
impl PD_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_11` writer - "]
pub struct PD_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_11_W<'a> {
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
#[doc = "Field `PD_12` reader - "]
pub struct PD_12_R(crate::FieldReader<bool, bool>);
impl PD_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_12` writer - "]
pub struct PD_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_12_W<'a> {
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
#[doc = "Field `PD_13` reader - "]
pub struct PD_13_R(crate::FieldReader<bool, bool>);
impl PD_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_13` writer - "]
pub struct PD_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_13_W<'a> {
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
#[doc = "Field `PD_14` reader - "]
pub struct PD_14_R(crate::FieldReader<bool, bool>);
impl PD_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_14` writer - "]
pub struct PD_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_14_W<'a> {
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
#[doc = "Field `PD_15` reader - "]
pub struct PD_15_R(crate::FieldReader<bool, bool>);
impl PD_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD_15` writer - "]
pub struct PD_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_15_W<'a> {
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
#[doc = "Field `SHM_0` reader - "]
pub struct SHM_0_R(crate::FieldReader<bool, bool>);
impl SHM_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_0` writer - "]
pub struct SHM_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_0_W<'a> {
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
#[doc = "Field `SHM_1` reader - "]
pub struct SHM_1_R(crate::FieldReader<bool, bool>);
impl SHM_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_1` writer - "]
pub struct SHM_1_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_1_W<'a> {
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
#[doc = "Field `SHM_2` reader - "]
pub struct SHM_2_R(crate::FieldReader<bool, bool>);
impl SHM_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_2` writer - "]
pub struct SHM_2_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_2_W<'a> {
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
#[doc = "Field `SHM_3` reader - "]
pub struct SHM_3_R(crate::FieldReader<bool, bool>);
impl SHM_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_3` writer - "]
pub struct SHM_3_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_3_W<'a> {
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
#[doc = "Field `SHM_4` reader - "]
pub struct SHM_4_R(crate::FieldReader<bool, bool>);
impl SHM_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_4` writer - "]
pub struct SHM_4_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_4_W<'a> {
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
#[doc = "Field `SHM_5` reader - "]
pub struct SHM_5_R(crate::FieldReader<bool, bool>);
impl SHM_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_5` writer - "]
pub struct SHM_5_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_5_W<'a> {
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
#[doc = "Field `SHM_6` reader - "]
pub struct SHM_6_R(crate::FieldReader<bool, bool>);
impl SHM_6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_6` writer - "]
pub struct SHM_6_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_6_W<'a> {
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
#[doc = "Field `SHM_7` reader - "]
pub struct SHM_7_R(crate::FieldReader<bool, bool>);
impl SHM_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_7` writer - "]
pub struct SHM_7_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_7_W<'a> {
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
#[doc = "Field `SHM_8` reader - "]
pub struct SHM_8_R(crate::FieldReader<bool, bool>);
impl SHM_8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_8` writer - "]
pub struct SHM_8_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_8_W<'a> {
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
#[doc = "Field `SHM_9` reader - "]
pub struct SHM_9_R(crate::FieldReader<bool, bool>);
impl SHM_9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_9` writer - "]
pub struct SHM_9_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_9_W<'a> {
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
#[doc = "Field `SHM_10` reader - "]
pub struct SHM_10_R(crate::FieldReader<bool, bool>);
impl SHM_10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_10` writer - "]
pub struct SHM_10_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_10_W<'a> {
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
#[doc = "Field `SHM_11` reader - "]
pub struct SHM_11_R(crate::FieldReader<bool, bool>);
impl SHM_11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_11` writer - "]
pub struct SHM_11_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_11_W<'a> {
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
#[doc = "Field `SHM_12` reader - "]
pub struct SHM_12_R(crate::FieldReader<bool, bool>);
impl SHM_12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_12` writer - "]
pub struct SHM_12_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_12_W<'a> {
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
#[doc = "Field `SHM_13` reader - "]
pub struct SHM_13_R(crate::FieldReader<bool, bool>);
impl SHM_13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_13` writer - "]
pub struct SHM_13_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_13_W<'a> {
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
#[doc = "Field `SHM_14` reader - "]
pub struct SHM_14_R(crate::FieldReader<bool, bool>);
impl SHM_14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_14` writer - "]
pub struct SHM_14_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_14_W<'a> {
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
#[doc = "Field `SHM_15` reader - "]
pub struct SHM_15_R(crate::FieldReader<bool, bool>);
impl SHM_15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SHM_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHM_15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHM_15` writer - "]
pub struct SHM_15_W<'a> {
    w: &'a mut W,
}
impl<'a> SHM_15_W<'a> {
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
    pub fn pd_0(&self) -> PD_0_R {
        PD_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pd_1(&self) -> PD_1_R {
        PD_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pd_2(&self) -> PD_2_R {
        PD_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pd_3(&self) -> PD_3_R {
        PD_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd_4(&self) -> PD_4_R {
        PD_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pd_5(&self) -> PD_5_R {
        PD_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pd_6(&self) -> PD_6_R {
        PD_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pd_7(&self) -> PD_7_R {
        PD_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pd_8(&self) -> PD_8_R {
        PD_8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pd_9(&self) -> PD_9_R {
        PD_9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pd_10(&self) -> PD_10_R {
        PD_10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pd_11(&self) -> PD_11_R {
        PD_11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pd_12(&self) -> PD_12_R {
        PD_12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pd_13(&self) -> PD_13_R {
        PD_13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_14(&self) -> PD_14_R {
        PD_14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_15(&self) -> PD_15_R {
        PD_15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shm_0(&self) -> SHM_0_R {
        SHM_0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn shm_1(&self) -> SHM_1_R {
        SHM_1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn shm_2(&self) -> SHM_2_R {
        SHM_2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn shm_3(&self) -> SHM_3_R {
        SHM_3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn shm_4(&self) -> SHM_4_R {
        SHM_4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn shm_5(&self) -> SHM_5_R {
        SHM_5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn shm_6(&self) -> SHM_6_R {
        SHM_6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn shm_7(&self) -> SHM_7_R {
        SHM_7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn shm_8(&self) -> SHM_8_R {
        SHM_8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn shm_9(&self) -> SHM_9_R {
        SHM_9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn shm_10(&self) -> SHM_10_R {
        SHM_10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn shm_11(&self) -> SHM_11_R {
        SHM_11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn shm_12(&self) -> SHM_12_R {
        SHM_12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn shm_13(&self) -> SHM_13_R {
        SHM_13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn shm_14(&self) -> SHM_14_R {
        SHM_14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn shm_15(&self) -> SHM_15_R {
        SHM_15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pd_0(&mut self) -> PD_0_W {
        PD_0_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pd_1(&mut self) -> PD_1_W {
        PD_1_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pd_2(&mut self) -> PD_2_W {
        PD_2_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pd_3(&mut self) -> PD_3_W {
        PD_3_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pd_4(&mut self) -> PD_4_W {
        PD_4_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pd_5(&mut self) -> PD_5_W {
        PD_5_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pd_6(&mut self) -> PD_6_W {
        PD_6_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pd_7(&mut self) -> PD_7_W {
        PD_7_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pd_8(&mut self) -> PD_8_W {
        PD_8_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pd_9(&mut self) -> PD_9_W {
        PD_9_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pd_10(&mut self) -> PD_10_W {
        PD_10_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pd_11(&mut self) -> PD_11_W {
        PD_11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pd_12(&mut self) -> PD_12_W {
        PD_12_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pd_13(&mut self) -> PD_13_W {
        PD_13_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pd_14(&mut self) -> PD_14_W {
        PD_14_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pd_15(&mut self) -> PD_15_W {
        PD_15_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn shm_0(&mut self) -> SHM_0_W {
        SHM_0_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn shm_1(&mut self) -> SHM_1_W {
        SHM_1_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn shm_2(&mut self) -> SHM_2_W {
        SHM_2_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn shm_3(&mut self) -> SHM_3_W {
        SHM_3_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn shm_4(&mut self) -> SHM_4_W {
        SHM_4_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn shm_5(&mut self) -> SHM_5_W {
        SHM_5_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn shm_6(&mut self) -> SHM_6_W {
        SHM_6_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn shm_7(&mut self) -> SHM_7_W {
        SHM_7_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn shm_8(&mut self) -> SHM_8_W {
        SHM_8_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn shm_9(&mut self) -> SHM_9_W {
        SHM_9_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn shm_10(&mut self) -> SHM_10_W {
        SHM_10_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn shm_11(&mut self) -> SHM_11_W {
        SHM_11_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn shm_12(&mut self) -> SHM_12_W {
        SHM_12_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn shm_13(&mut self) -> SHM_13_W {
        SHM_13_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn shm_14(&mut self) -> SHM_14_W {
        SHM_14_W { w: self }
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn shm_15(&mut self) -> SHM_15_W {
        SHM_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Driver Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pd](index.html) module"]
pub struct PD_SPEC;
impl crate::RegisterSpec for PD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pd::R](R) reader structure"]
impl crate::Readable for PD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pd::W](W) writer structure"]
impl crate::Writable for PD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PD to value 0"]
impl crate::Resettable for PD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
