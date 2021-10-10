#[doc = "Register `HSCR` reader"]
pub struct R(crate::R<HSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCR` writer"]
pub struct W(crate::W<HSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCR_SPEC>;
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
impl From<crate::W<HSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_MODE` reader - "]
pub struct HOST_MODE_R(crate::FieldReader<bool, bool>);
impl HOST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_MODE` writer - "]
pub struct HOST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_MODE_W<'a> {
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
#[doc = "Field `RESET_CORE` reader - "]
pub struct RESET_CORE_R(crate::FieldReader<bool, bool>);
impl RESET_CORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_CORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_CORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET_CORE` writer - "]
pub struct RESET_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_CORE_W<'a> {
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
#[doc = "Field `EN_TX` reader - "]
pub struct EN_TX_R(crate::FieldReader<bool, bool>);
impl EN_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_TX` writer - "]
pub struct EN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_TX_W<'a> {
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
#[doc = "Field `EN_RX` reader - "]
pub struct EN_RX_R(crate::FieldReader<bool, bool>);
impl EN_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_RX` writer - "]
pub struct EN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_RX_W<'a> {
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
#[doc = "Field `DP_PULLUP` reader - "]
pub struct DP_PULLUP_R(crate::FieldReader<bool, bool>);
impl DP_PULLUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLUP` writer - "]
pub struct DP_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLUP_W<'a> {
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
#[doc = "Field `DP_PULLDOWN` reader - "]
pub struct DP_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl DP_PULLDOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DP_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLDOWN` writer - "]
pub struct DP_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDOWN_W<'a> {
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
#[doc = "Field `DM_PULLUP` reader - "]
pub struct DM_PULLUP_R(crate::FieldReader<bool, bool>);
impl DM_PULLUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLUP` writer - "]
pub struct DM_PULLUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLUP_W<'a> {
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
#[doc = "Field `DM_PULLDOWN` reader - "]
pub struct DM_PULLDOWN_R(crate::FieldReader<bool, bool>);
impl DM_PULLDOWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DM_PULLDOWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLDOWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLDOWN` writer - "]
pub struct DM_PULLDOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDOWN_W<'a> {
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
    pub fn host_mode(&self) -> HOST_MODE_R {
        HOST_MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reset_core(&self) -> RESET_CORE_R {
        RESET_CORE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_tx(&self) -> EN_TX_R {
        EN_TX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn en_rx(&self) -> EN_RX_R {
        EN_RX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dp_pullup(&self) -> DP_PULLUP_R {
        DP_PULLUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dp_pulldown(&self) -> DP_PULLDOWN_R {
        DP_PULLDOWN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dm_pullup(&self) -> DM_PULLUP_R {
        DM_PULLUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dm_pulldown(&self) -> DM_PULLDOWN_R {
        DM_PULLDOWN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn host_mode(&mut self) -> HOST_MODE_W {
        HOST_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reset_core(&mut self) -> RESET_CORE_W {
        RESET_CORE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn en_tx(&mut self) -> EN_TX_W {
        EN_TX_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn en_rx(&mut self) -> EN_RX_W {
        EN_RX_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dp_pullup(&mut self) -> DP_PULLUP_W {
        DP_PULLUP_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dp_pulldown(&mut self) -> DP_PULLDOWN_W {
        DP_PULLDOWN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dm_pullup(&mut self) -> DM_PULLUP_W {
        DM_PULLUP_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dm_pulldown(&mut self) -> DM_PULLDOWN_W {
        DM_PULLDOWN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HSCR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscr](index.html) module"]
pub struct HSCR_SPEC;
impl crate::RegisterSpec for HSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hscr::R](R) reader structure"]
impl crate::Readable for HSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hscr::W](W) writer structure"]
impl crate::Writable for HSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCR to value 0"]
impl crate::Resettable for HSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
