#[doc = "Register `FUNC` reader"]
pub struct R(crate::R<FUNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC` writer"]
pub struct W(crate::W<FUNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_SPEC>;
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
impl From<crate::W<FUNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE_0` reader - "]
pub struct MODE_0_R(crate::FieldReader<u8, u8>);
impl MODE_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_0` writer - "]
pub struct MODE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `MODE_1` reader - "]
pub struct MODE_1_R(crate::FieldReader<u8, u8>);
impl MODE_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_1` writer - "]
pub struct MODE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `MODE_2` reader - "]
pub struct MODE_2_R(crate::FieldReader<u8, u8>);
impl MODE_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_2` writer - "]
pub struct MODE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `MODE_3` reader - "]
pub struct MODE_3_R(crate::FieldReader<u8, u8>);
impl MODE_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_3` writer - "]
pub struct MODE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `MODE_4` reader - "]
pub struct MODE_4_R(crate::FieldReader<u8, u8>);
impl MODE_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_4` writer - "]
pub struct MODE_4_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `MODE_5` reader - "]
pub struct MODE_5_R(crate::FieldReader<u8, u8>);
impl MODE_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_5` writer - "]
pub struct MODE_5_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `MODE_6` reader - "]
pub struct MODE_6_R(crate::FieldReader<u8, u8>);
impl MODE_6_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_6` writer - "]
pub struct MODE_6_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `MODE_7` reader - "]
pub struct MODE_7_R(crate::FieldReader<u8, u8>);
impl MODE_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_7` writer - "]
pub struct MODE_7_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `MODE_8` reader - "]
pub struct MODE_8_R(crate::FieldReader<u8, u8>);
impl MODE_8_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_8` writer - "]
pub struct MODE_8_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `MODE_9` reader - "]
pub struct MODE_9_R(crate::FieldReader<u8, u8>);
impl MODE_9_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_9` writer - "]
pub struct MODE_9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `MODE_10` reader - "]
pub struct MODE_10_R(crate::FieldReader<u8, u8>);
impl MODE_10_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_10` writer - "]
pub struct MODE_10_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `MODE_11` reader - "]
pub struct MODE_11_R(crate::FieldReader<u8, u8>);
impl MODE_11_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_11` writer - "]
pub struct MODE_11_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `MODE_12` reader - "]
pub struct MODE_12_R(crate::FieldReader<u8, u8>);
impl MODE_12_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_12` writer - "]
pub struct MODE_12_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `MODE_13` reader - "]
pub struct MODE_13_R(crate::FieldReader<u8, u8>);
impl MODE_13_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_13` writer - "]
pub struct MODE_13_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `MODE_14` reader - "]
pub struct MODE_14_R(crate::FieldReader<u8, u8>);
impl MODE_14_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_14` writer - "]
pub struct MODE_14_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `MODE_15` reader - "]
pub struct MODE_15_R(crate::FieldReader<u8, u8>);
impl MODE_15_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_15` writer - "]
pub struct MODE_15_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mode_0(&self) -> MODE_0_R {
        MODE_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mode_1(&self) -> MODE_1_R {
        MODE_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mode_2(&self) -> MODE_2_R {
        MODE_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn mode_3(&self) -> MODE_3_R {
        MODE_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode_4(&self) -> MODE_4_R {
        MODE_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn mode_5(&self) -> MODE_5_R {
        MODE_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mode_6(&self) -> MODE_6_R {
        MODE_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn mode_7(&self) -> MODE_7_R {
        MODE_7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn mode_8(&self) -> MODE_8_R {
        MODE_8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn mode_9(&self) -> MODE_9_R {
        MODE_9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn mode_10(&self) -> MODE_10_R {
        MODE_10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mode_11(&self) -> MODE_11_R {
        MODE_11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode_12(&self) -> MODE_12_R {
        MODE_12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn mode_13(&self) -> MODE_13_R {
        MODE_13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn mode_14(&self) -> MODE_14_R {
        MODE_14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn mode_15(&self) -> MODE_15_R {
        MODE_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn mode_0(&mut self) -> MODE_0_W {
        MODE_0_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mode_1(&mut self) -> MODE_1_W {
        MODE_1_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn mode_2(&mut self) -> MODE_2_W {
        MODE_2_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn mode_3(&mut self) -> MODE_3_W {
        MODE_3_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn mode_4(&mut self) -> MODE_4_W {
        MODE_4_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn mode_5(&mut self) -> MODE_5_W {
        MODE_5_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn mode_6(&mut self) -> MODE_6_W {
        MODE_6_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn mode_7(&mut self) -> MODE_7_W {
        MODE_7_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn mode_8(&mut self) -> MODE_8_W {
        MODE_8_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn mode_9(&mut self) -> MODE_9_W {
        MODE_9_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn mode_10(&mut self) -> MODE_10_W {
        MODE_10_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn mode_11(&mut self) -> MODE_11_W {
        MODE_11_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn mode_12(&mut self) -> MODE_12_W {
        MODE_12_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn mode_13(&mut self) -> MODE_13_W {
        MODE_13_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn mode_14(&mut self) -> MODE_14_W {
        MODE_14_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn mode_15(&mut self) -> MODE_15_W {
        MODE_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Function Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func](index.html) module"]
pub struct FUNC_SPEC;
impl crate::RegisterSpec for FUNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func::R](R) reader structure"]
impl crate::Readable for FUNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func::W](W) writer structure"]
impl crate::Writable for FUNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC to value 0"]
impl crate::Resettable for FUNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
