#[doc = "Register `DMA_RE` reader"]
pub struct R(crate::R<DMA_RE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_RE` writer"]
pub struct W(crate::W<DMA_RE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_RE_SPEC>;
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
impl From<crate::W<DMA_RE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_RE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_ZERO_EVENT_RE` reader - "]
pub struct CNT_ZERO_EVENT_RE_R(crate::FieldReader<bool, bool>);
impl CNT_ZERO_EVENT_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNT_ZERO_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_ZERO_EVENT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_ZERO_EVENT_RE` writer - "]
pub struct CNT_ZERO_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_ZERO_EVENT_RE_W<'a> {
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
#[doc = "Field `CNT_ARR_EVENT_RE` reader - "]
pub struct CNT_ARR_EVENT_RE_R(crate::FieldReader<bool, bool>);
impl CNT_ARR_EVENT_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNT_ARR_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNT_ARR_EVENT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNT_ARR_EVENT_RE` writer - "]
pub struct CNT_ARR_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CNT_ARR_EVENT_RE_W<'a> {
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
#[doc = "Field `ETR_RE_EVENT_RE` reader - "]
pub struct ETR_RE_EVENT_RE_R(crate::FieldReader<bool, bool>);
impl ETR_RE_EVENT_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETR_RE_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_RE_EVENT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_RE_EVENT_RE` writer - "]
pub struct ETR_RE_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_RE_EVENT_RE_W<'a> {
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
#[doc = "Field `ETR_FE_EVENT_RE` reader - "]
pub struct ETR_FE_EVENT_RE_R(crate::FieldReader<bool, bool>);
impl ETR_FE_EVENT_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETR_FE_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ETR_FE_EVENT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ETR_FE_EVENT_RE` writer - "]
pub struct ETR_FE_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_FE_EVENT_RE_W<'a> {
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
#[doc = "Field `BRK_EVENT_RE` reader - "]
pub struct BRK_EVENT_RE_R(crate::FieldReader<bool, bool>);
impl BRK_EVENT_RE_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRK_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_EVENT_RE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK_EVENT_RE` writer - "]
pub struct BRK_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_EVENT_RE_W<'a> {
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
#[doc = "Field `CCR_CAP_EVENT_RE` reader - "]
pub struct CCR_CAP_EVENT_RE_R(crate::FieldReader<u8, u8>);
impl CCR_CAP_EVENT_RE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCR_CAP_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR_CAP_EVENT_RE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR_CAP_EVENT_RE` writer - "]
pub struct CCR_CAP_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_CAP_EVENT_RE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `CCR_REF_EVENT_RE` reader - "]
pub struct CCR_REF_EVENT_RE_R(crate::FieldReader<u8, u8>);
impl CCR_REF_EVENT_RE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCR_REF_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR_REF_EVENT_RE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR_REF_EVENT_RE` writer - "]
pub struct CCR_REF_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR_REF_EVENT_RE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | ((value as u32 & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `CCR1_CAP_EVENT_RE` reader - "]
pub struct CCR1_CAP_EVENT_RE_R(crate::FieldReader<u8, u8>);
impl CCR1_CAP_EVENT_RE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCR1_CAP_EVENT_RE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCR1_CAP_EVENT_RE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCR1_CAP_EVENT_RE` writer - "]
pub struct CCR1_CAP_EVENT_RE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCR1_CAP_EVENT_RE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 13)) | ((value as u32 & 0x0f) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_zero_event_re(&self) -> CNT_ZERO_EVENT_RE_R {
        CNT_ZERO_EVENT_RE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnt_arr_event_re(&self) -> CNT_ARR_EVENT_RE_R {
        CNT_ARR_EVENT_RE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn etr_re_event_re(&self) -> ETR_RE_EVENT_RE_R {
        ETR_RE_EVENT_RE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etr_fe_event_re(&self) -> ETR_FE_EVENT_RE_R {
        ETR_FE_EVENT_RE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn brk_event_re(&self) -> BRK_EVENT_RE_R {
        BRK_EVENT_RE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn ccr_cap_event_re(&self) -> CCR_CAP_EVENT_RE_R {
        CCR_CAP_EVENT_RE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn ccr_ref_event_re(&self) -> CCR_REF_EVENT_RE_R {
        CCR_REF_EVENT_RE_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn ccr1_cap_event_re(&self) -> CCR1_CAP_EVENT_RE_R {
        CCR1_CAP_EVENT_RE_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnt_zero_event_re(&mut self) -> CNT_ZERO_EVENT_RE_W {
        CNT_ZERO_EVENT_RE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cnt_arr_event_re(&mut self) -> CNT_ARR_EVENT_RE_W {
        CNT_ARR_EVENT_RE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn etr_re_event_re(&mut self) -> ETR_RE_EVENT_RE_W {
        ETR_RE_EVENT_RE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn etr_fe_event_re(&mut self) -> ETR_FE_EVENT_RE_W {
        ETR_FE_EVENT_RE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn brk_event_re(&mut self) -> BRK_EVENT_RE_W {
        BRK_EVENT_RE_W { w: self }
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn ccr_cap_event_re(&mut self) -> CCR_CAP_EVENT_RE_W {
        CCR_CAP_EVENT_RE_W { w: self }
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn ccr_ref_event_re(&mut self) -> CCR_REF_EVENT_RE_W {
        CCR_REF_EVENT_RE_W { w: self }
    }
    #[doc = "Bits 13:16"]
    #[inline(always)]
    pub fn ccr1_cap_event_re(&mut self) -> CCR1_CAP_EVENT_RE_W {
        CCR1_CAP_EVENT_RE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer DMA Request Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_re](index.html) module"]
pub struct DMA_RE_SPEC;
impl crate::RegisterSpec for DMA_RE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_re::R](R) reader structure"]
impl crate::Readable for DMA_RE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_re::W](W) writer structure"]
impl crate::Writable for DMA_RE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_RE to value 0"]
impl crate::Resettable for DMA_RE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
