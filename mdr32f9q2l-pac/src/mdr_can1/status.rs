#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_READY` reader - "]
pub struct RX_READY_R(crate::FieldReader<bool, bool>);
impl RX_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_READY` writer - "]
pub struct RX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_READY_W<'a> {
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
#[doc = "Field `TX_READY` reader - "]
pub struct TX_READY_R(crate::FieldReader<bool, bool>);
impl TX_READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_READY` writer - "]
pub struct TX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_W<'a> {
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
#[doc = "Field `ERROR_OVER` reader - "]
pub struct ERROR_OVER_R(crate::FieldReader<bool, bool>);
impl ERROR_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERROR_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERROR_OVER` writer - "]
pub struct ERROR_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_OVER_W<'a> {
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
#[doc = "Field `BIT_ERR` reader - "]
pub struct BIT_ERR_R(crate::FieldReader<bool, bool>);
impl BIT_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_ERR` writer - "]
pub struct BIT_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_ERR_W<'a> {
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
#[doc = "Field `BIT_STUFF_ERR` reader - "]
pub struct BIT_STUFF_ERR_R(crate::FieldReader<bool, bool>);
impl BIT_STUFF_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIT_STUFF_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_STUFF_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIT_STUFF_ERR` writer - "]
pub struct BIT_STUFF_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_STUFF_ERR_W<'a> {
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
#[doc = "Field `CRC_ERR` reader - "]
pub struct CRC_ERR_R(crate::FieldReader<bool, bool>);
impl CRC_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRC_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC_ERR` writer - "]
pub struct CRC_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_ERR_W<'a> {
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
#[doc = "Field `FRAME_ERR` reader - "]
pub struct FRAME_ERR_R(crate::FieldReader<bool, bool>);
impl FRAME_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRAME_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRAME_ERR` writer - "]
pub struct FRAME_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRAME_ERR_W<'a> {
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
#[doc = "Field `ACK_ERR` reader - "]
pub struct ACK_ERR_R(crate::FieldReader<bool, bool>);
impl ACK_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACK_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACK_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_ERR` writer - "]
pub struct ACK_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_ERR_W<'a> {
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
#[doc = "Field `IDLOWER` reader - "]
pub struct IDLOWER_R(crate::FieldReader<bool, bool>);
impl IDLOWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDLOWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDLOWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDLOWER` writer - "]
pub struct IDLOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLOWER_W<'a> {
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
#[doc = "Field `ERR_STATUS` reader - "]
pub struct ERR_STATUS_R(crate::FieldReader<u8, u8>);
impl ERR_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ERR_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_STATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_STATUS` writer - "]
pub struct ERR_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `RX_ERR_CNT8` reader - "]
pub struct RX_ERR_CNT8_R(crate::FieldReader<bool, bool>);
impl RX_ERR_CNT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_ERR_CNT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_CNT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_CNT8` writer - "]
pub struct RX_ERR_CNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_CNT8_W<'a> {
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
#[doc = "Field `TX_ERR_CNT8` reader - "]
pub struct TX_ERR_CNT8_R(crate::FieldReader<bool, bool>);
impl TX_ERR_CNT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_ERR_CNT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ERR_CNT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ERR_CNT8` writer - "]
pub struct TX_ERR_CNT8_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ERR_CNT8_W<'a> {
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
#[doc = "Field `RX_ERR_CNT` reader - "]
pub struct RX_ERR_CNT_R(crate::FieldReader<u8, u8>);
impl RX_ERR_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_ERR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_ERR_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_ERR_CNT` writer - "]
pub struct RX_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `TX_ERR_CNT` reader - "]
pub struct TX_ERR_CNT_R(crate::FieldReader<u8, u8>);
impl TX_ERR_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_ERR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ERR_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_ERR_CNT` writer - "]
pub struct TX_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_ready(&self) -> RX_READY_R {
        RX_READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_over(&self) -> ERROR_OVER_R {
        ERROR_OVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bit_err(&self) -> BIT_ERR_R {
        BIT_ERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bit_stuff_err(&self) -> BIT_STUFF_ERR_R {
        BIT_STUFF_ERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc_err(&self) -> CRC_ERR_R {
        CRC_ERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frame_err(&self) -> FRAME_ERR_R {
        FRAME_ERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ack_err(&self) -> ACK_ERR_R {
        ACK_ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn idlower(&self) -> IDLOWER_R {
        IDLOWER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn err_status(&self) -> ERR_STATUS_R {
        ERR_STATUS_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_err_cnt8(&self) -> RX_ERR_CNT8_R {
        RX_ERR_CNT8_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_err_cnt8(&self) -> TX_ERR_CNT8_R {
        TX_ERR_CNT8_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rx_err_cnt(&self) -> RX_ERR_CNT_R {
        RX_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_err_cnt(&self) -> TX_ERR_CNT_R {
        TX_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rx_ready(&mut self) -> RX_READY_W {
        RX_READY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn error_over(&mut self) -> ERROR_OVER_W {
        ERROR_OVER_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn bit_err(&mut self) -> BIT_ERR_W {
        BIT_ERR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn bit_stuff_err(&mut self) -> BIT_STUFF_ERR_W {
        BIT_STUFF_ERR_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn crc_err(&mut self) -> CRC_ERR_W {
        CRC_ERR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn frame_err(&mut self) -> FRAME_ERR_W {
        FRAME_ERR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ack_err(&mut self) -> ACK_ERR_W {
        ACK_ERR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn idlower(&mut self) -> IDLOWER_W {
        IDLOWER_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn err_status(&mut self) -> ERR_STATUS_W {
        ERR_STATUS_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rx_err_cnt8(&mut self) -> RX_ERR_CNT8_W {
        RX_ERR_CNT8_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tx_err_cnt8(&mut self) -> TX_ERR_CNT8_W {
        TX_ERR_CNT8_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn rx_err_cnt(&mut self) -> RX_ERR_CNT_W {
        RX_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn tx_err_cnt(&mut self) -> TX_ERR_CNT_W {
        TX_ERR_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
