#[doc = "Register `BUF_14_DATAH` reader"]
pub struct R(crate::R<BUF_14_DATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_14_DATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_14_DATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_14_DATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_14_DATAH` writer"]
pub struct W(crate::W<BUF_14_DATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_14_DATAH_SPEC>;
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
impl From<crate::W<BUF_14_DATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_14_DATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DB4` reader - "]
pub struct DB4_R(crate::FieldReader<u8, u8>);
impl DB4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB4` writer - "]
pub struct DB4_W<'a> {
    w: &'a mut W,
}
impl<'a> DB4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DB5` reader - "]
pub struct DB5_R(crate::FieldReader<u8, u8>);
impl DB5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB5` writer - "]
pub struct DB5_W<'a> {
    w: &'a mut W,
}
impl<'a> DB5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DB6` reader - "]
pub struct DB6_R(crate::FieldReader<u8, u8>);
impl DB6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB6` writer - "]
pub struct DB6_W<'a> {
    w: &'a mut W,
}
impl<'a> DB6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DB7` reader - "]
pub struct DB7_R(crate::FieldReader<u8, u8>);
impl DB7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DB7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DB7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DB7` writer - "]
pub struct DB7_W<'a> {
    w: &'a mut W,
}
impl<'a> DB7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn db4(&self) -> DB4_R {
        DB4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn db5(&self) -> DB5_R {
        DB5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn db6(&self) -> DB6_R {
        DB6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn db7(&self) -> DB7_R {
        DB7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn db4(&mut self) -> DB4_W {
        DB4_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn db5(&mut self) -> DB5_W {
        DB5_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn db6(&mut self) -> DB6_W {
        DB6_W { w: self }
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn db7(&mut self) -> DB7_W {
        DB7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAN Buffer Data high Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_14_datah](index.html) module"]
pub struct BUF_14_DATAH_SPEC;
impl crate::RegisterSpec for BUF_14_DATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_14_datah::R](R) reader structure"]
impl crate::Readable for BUF_14_DATAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_14_datah::W](W) writer structure"]
impl crate::Writable for BUF_14_DATAH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_14_DATAH to value 0"]
impl crate::Resettable for BUF_14_DATAH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
