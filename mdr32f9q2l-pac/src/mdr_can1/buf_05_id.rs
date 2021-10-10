#[doc = "Register `BUF_05_ID` reader"]
pub struct R(crate::R<BUF_05_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_05_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_05_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_05_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_05_ID` writer"]
pub struct W(crate::W<BUF_05_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_05_ID_SPEC>;
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
impl From<crate::W<BUF_05_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_05_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EID` reader - "]
pub struct EID_R(crate::FieldReader<u32, u32>);
impl EID_R {
    pub(crate) fn new(bits: u32) -> Self {
        EID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EID_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EID` writer - "]
pub struct EID_W<'a> {
    w: &'a mut W,
}
impl<'a> EID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
#[doc = "Field `SID` reader - "]
pub struct SID_R(crate::FieldReader<u16, u16>);
impl SID_R {
    pub(crate) fn new(bits: u16) -> Self {
        SID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SID` writer - "]
pub struct SID_W<'a> {
    w: &'a mut W,
}
impl<'a> SID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 18)) | ((value as u32 & 0x07ff) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn eid(&self) -> EID_R {
        EID_R::new((self.bits & 0x0003_ffff) as u32)
    }
    #[doc = "Bits 18:28"]
    #[inline(always)]
    pub fn sid(&self) -> SID_R {
        SID_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn eid(&mut self) -> EID_W {
        EID_W { w: self }
    }
    #[doc = "Bits 18:28"]
    #[inline(always)]
    pub fn sid(&mut self) -> SID_W {
        SID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_05_id](index.html) module"]
pub struct BUF_05_ID_SPEC;
impl crate::RegisterSpec for BUF_05_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_05_id::R](R) reader structure"]
impl crate::Readable for BUF_05_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_05_id::W](W) writer structure"]
impl crate::Writable for BUF_05_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_05_ID to value 0"]
impl crate::Resettable for BUF_05_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
