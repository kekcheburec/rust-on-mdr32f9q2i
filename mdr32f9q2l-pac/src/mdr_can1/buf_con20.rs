#[doc = "Register `BUF_CON20` reader"]
pub struct R(crate::R<BUF_CON20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_CON20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_CON20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_CON20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_CON20` writer"]
pub struct W(crate::W<BUF_CON20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_CON20_SPEC>;
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
impl From<crate::W<BUF_CON20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_CON20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - "]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - "]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Field `RX_TXn` reader - "]
pub struct RX_TXN_R(crate::FieldReader<bool, bool>);
impl RX_TXN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_TXN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TXN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_TXn` writer - "]
pub struct RX_TXN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TXN_W<'a> {
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
#[doc = "Field `OVER_EN` reader - "]
pub struct OVER_EN_R(crate::FieldReader<bool, bool>);
impl OVER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_EN` writer - "]
pub struct OVER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_EN_W<'a> {
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
#[doc = "Field `RTR_EN` reader - "]
pub struct RTR_EN_R(crate::FieldReader<bool, bool>);
impl RTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTR_EN` writer - "]
pub struct RTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_EN_W<'a> {
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
#[doc = "Field `PRIOR_0` reader - "]
pub struct PRIOR_0_R(crate::FieldReader<bool, bool>);
impl PRIOR_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRIOR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIOR_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIOR_0` writer - "]
pub struct PRIOR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIOR_0_W<'a> {
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
#[doc = "Field `TX_REQ` reader - "]
pub struct TX_REQ_R(crate::FieldReader<bool, bool>);
impl TX_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_REQ` writer - "]
pub struct TX_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_REQ_W<'a> {
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
#[doc = "Field `RX_FULL` reader - "]
pub struct RX_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_FULL` writer - "]
pub struct RX_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FULL_W<'a> {
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
#[doc = "Field `OVER_WR` reader - "]
pub struct OVER_WR_R(crate::FieldReader<bool, bool>);
impl OVER_WR_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVER_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVER_WR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVER_WR` writer - "]
pub struct OVER_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_WR_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_txn(&self) -> RX_TXN_R {
        RX_TXN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn over_en(&self) -> OVER_EN_R {
        OVER_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtr_en(&self) -> RTR_EN_R {
        RTR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prior_0(&self) -> PRIOR_0_R {
        PRIOR_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_req(&self) -> TX_REQ_R {
        TX_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_full(&self) -> RX_FULL_R {
        RX_FULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn over_wr(&self) -> OVER_WR_R {
        OVER_WR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_txn(&mut self) -> RX_TXN_W {
        RX_TXN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn over_en(&mut self) -> OVER_EN_W {
        OVER_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rtr_en(&mut self) -> RTR_EN_W {
        RTR_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn prior_0(&mut self) -> PRIOR_0_W {
        PRIOR_0_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_req(&mut self) -> TX_REQ_W {
        TX_REQ_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_full(&mut self) -> RX_FULL_W {
        RX_FULL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn over_wr(&mut self) -> OVER_WR_W {
        OVER_WR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer Connection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_con20](index.html) module"]
pub struct BUF_CON20_SPEC;
impl crate::RegisterSpec for BUF_CON20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_con20::R](R) reader structure"]
impl crate::Readable for BUF_CON20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_con20::W](W) writer structure"]
impl crate::Writable for BUF_CON20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_CON20 to value 0"]
impl crate::Resettable for BUF_CON20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
