#[doc = "Register `HSVR` reader"]
pub struct R(crate::R<HSVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSVR` writer"]
pub struct W(crate::W<HSVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSVR_SPEC>;
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
impl From<crate::W<HSVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - "]
pub struct VERSION_R(crate::FieldReader<u8, u8>);
impl VERSION_R {
    pub(crate) fn new(bits: u8) -> Self {
        VERSION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VERSION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VERSION` writer - "]
pub struct VERSION_W<'a> {
    w: &'a mut W,
}
impl<'a> VERSION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `REVISION` reader - "]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` writer - "]
pub struct REVISION_W<'a> {
    w: &'a mut W,
}
impl<'a> REVISION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn version(&mut self) -> VERSION_W {
        VERSION_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn revision(&mut self) -> REVISION_W {
        REVISION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_HSVR Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsvr](index.html) module"]
pub struct HSVR_SPEC;
impl crate::RegisterSpec for HSVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsvr::R](R) reader structure"]
impl crate::Readable for HSVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsvr::W](W) writer structure"]
impl crate::Writable for HSVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSVR to value 0"]
impl crate::Resettable for HSVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
