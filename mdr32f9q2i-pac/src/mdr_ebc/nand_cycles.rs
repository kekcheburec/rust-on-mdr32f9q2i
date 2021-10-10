#[doc = "Register `NAND_CYCLES` reader"]
pub struct R(crate::R<NAND_CYCLES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NAND_CYCLES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NAND_CYCLES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NAND_CYCLES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NAND_CYCLES` writer"]
pub struct W(crate::W<NAND_CYCLES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NAND_CYCLES_SPEC>;
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
impl From<crate::W<NAND_CYCLES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NAND_CYCLES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRC` reader - "]
pub struct TRC_R(crate::FieldReader<u8, u8>);
impl TRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRC` writer - "]
pub struct TRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `TWC` reader - "]
pub struct TWC_R(crate::FieldReader<u8, u8>);
impl TWC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWC` writer - "]
pub struct TWC_W<'a> {
    w: &'a mut W,
}
impl<'a> TWC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `TREA` reader - "]
pub struct TREA_R(crate::FieldReader<u8, u8>);
impl TREA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TREA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TREA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TREA` writer - "]
pub struct TREA_W<'a> {
    w: &'a mut W,
}
impl<'a> TREA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `TWP` reader - "]
pub struct TWP_R(crate::FieldReader<u8, u8>);
impl TWP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWP` writer - "]
pub struct TWP_W<'a> {
    w: &'a mut W,
}
impl<'a> TWP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `TWHR` reader - "]
pub struct TWHR_R(crate::FieldReader<u8, u8>);
impl TWHR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TWHR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWHR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWHR` writer - "]
pub struct TWHR_W<'a> {
    w: &'a mut W,
}
impl<'a> TWHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `TALEA` reader - "]
pub struct TALEA_R(crate::FieldReader<u8, u8>);
impl TALEA_R {
    pub(crate) fn new(bits: u8) -> Self {
        TALEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TALEA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TALEA` writer - "]
pub struct TALEA_W<'a> {
    w: &'a mut W,
}
impl<'a> TALEA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `TRR` reader - "]
pub struct TRR_R(crate::FieldReader<u8, u8>);
impl TRR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRR` writer - "]
pub struct TRR_W<'a> {
    w: &'a mut W,
}
impl<'a> TRR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn twc(&self) -> TWC_R {
        TWC_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn trea(&self) -> TREA_R {
        TREA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn twp(&self) -> TWP_R {
        TWP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn twhr(&self) -> TWHR_R {
        TWHR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn talea(&self) -> TALEA_R {
        TALEA_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W {
        TRC_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn twc(&mut self) -> TWC_W {
        TWC_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn trea(&mut self) -> TREA_W {
        TREA_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn twp(&mut self) -> TWP_W {
        TWP_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn twhr(&mut self) -> TWHR_W {
        TWHR_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn talea(&mut self) -> TALEA_W {
        TALEA_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn trr(&mut self) -> TRR_W {
        TRR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EBC NAND Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nand_cycles](index.html) module"]
pub struct NAND_CYCLES_SPEC;
impl crate::RegisterSpec for NAND_CYCLES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nand_cycles::R](R) reader structure"]
impl crate::Readable for NAND_CYCLES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nand_cycles::W](W) writer structure"]
impl crate::Writable for NAND_CYCLES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NAND_CYCLES to value 0"]
impl crate::Resettable for NAND_CYCLES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
