#[doc = "Register `INT_EN` reader"]
pub struct R(crate::R<INT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_EN` writer"]
pub struct W(crate::W<INT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_EN_SPEC>;
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
impl From<crate::W<INT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GLB_INT_EN` reader - "]
pub struct GLB_INT_EN_R(crate::FieldReader<bool, bool>);
impl GLB_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLB_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GLB_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLB_INT_EN` writer - "]
pub struct GLB_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLB_INT_EN_W<'a> {
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
#[doc = "Field `RX_INT_EN` reader - "]
pub struct RX_INT_EN_R(crate::FieldReader<bool, bool>);
impl RX_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RX_INT_EN` writer - "]
pub struct RX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_INT_EN_W<'a> {
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
#[doc = "Field `TX_INT_EN` reader - "]
pub struct TX_INT_EN_R(crate::FieldReader<bool, bool>);
impl TX_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_INT_EN` writer - "]
pub struct TX_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_INT_EN_W<'a> {
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
#[doc = "Field `ERR_INT_EN` reader - "]
pub struct ERR_INT_EN_R(crate::FieldReader<bool, bool>);
impl ERR_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_INT_EN` writer - "]
pub struct ERR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_INT_EN_W<'a> {
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
#[doc = "Field `ERR_OVER_INT_EN` reader - "]
pub struct ERR_OVER_INT_EN_R(crate::FieldReader<bool, bool>);
impl ERR_OVER_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_OVER_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_OVER_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR_OVER_INT_EN` writer - "]
pub struct ERR_OVER_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_OVER_INT_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn glb_int_en(&self) -> GLB_INT_EN_R {
        GLB_INT_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_int_en(&self) -> RX_INT_EN_R {
        RX_INT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_int_en(&self) -> TX_INT_EN_R {
        TX_INT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn err_int_en(&self) -> ERR_INT_EN_R {
        ERR_INT_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn err_over_int_en(&self) -> ERR_OVER_INT_EN_R {
        ERR_OVER_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn glb_int_en(&mut self) -> GLB_INT_EN_W {
        GLB_INT_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_int_en(&mut self) -> RX_INT_EN_W {
        RX_INT_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_int_en(&mut self) -> TX_INT_EN_W {
        TX_INT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn err_int_en(&mut self) -> ERR_INT_EN_W {
        ERR_INT_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn err_over_int_en(&mut self) -> ERR_OVER_INT_EN_W {
        ERR_OVER_INT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Interrupt enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_en](index.html) module"]
pub struct INT_EN_SPEC;
impl crate::RegisterSpec for INT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_en::R](R) reader structure"]
impl crate::Readable for INT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_en::W](W) writer structure"]
impl crate::Writable for INT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_EN to value 0"]
impl crate::Resettable for INT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
