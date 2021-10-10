#[doc = "Register `PWR` reader"]
pub struct R(crate::R<PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR` writer"]
pub struct W(crate::W<PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SPEC>;
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
impl From<crate::W<PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWR_0` reader - "]
pub struct PWR_0_R(crate::FieldReader<u8, u8>);
impl PWR_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_0` writer - "]
pub struct PWR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PWR_1` reader - "]
pub struct PWR_1_R(crate::FieldReader<u8, u8>);
impl PWR_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_1` writer - "]
pub struct PWR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `PWR_2` reader - "]
pub struct PWR_2_R(crate::FieldReader<u8, u8>);
impl PWR_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_2` writer - "]
pub struct PWR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `PWR_3` reader - "]
pub struct PWR_3_R(crate::FieldReader<u8, u8>);
impl PWR_3_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_3` writer - "]
pub struct PWR_3_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PWR_4` reader - "]
pub struct PWR_4_R(crate::FieldReader<u8, u8>);
impl PWR_4_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_4` writer - "]
pub struct PWR_4_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `PWR_5` reader - "]
pub struct PWR_5_R(crate::FieldReader<u8, u8>);
impl PWR_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_5` writer - "]
pub struct PWR_5_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `PWR_6` reader - "]
pub struct PWR_6_R(crate::FieldReader<u8, u8>);
impl PWR_6_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_6` writer - "]
pub struct PWR_6_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `PWR_7` reader - "]
pub struct PWR_7_R(crate::FieldReader<u8, u8>);
impl PWR_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_7` writer - "]
pub struct PWR_7_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `PWR_8` reader - "]
pub struct PWR_8_R(crate::FieldReader<u8, u8>);
impl PWR_8_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_8` writer - "]
pub struct PWR_8_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `PWR_9` reader - "]
pub struct PWR_9_R(crate::FieldReader<u8, u8>);
impl PWR_9_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_9` writer - "]
pub struct PWR_9_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Field `PWR_10` reader - "]
pub struct PWR_10_R(crate::FieldReader<u8, u8>);
impl PWR_10_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_10` writer - "]
pub struct PWR_10_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `PWR_11` reader - "]
pub struct PWR_11_R(crate::FieldReader<u8, u8>);
impl PWR_11_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_11` writer - "]
pub struct PWR_11_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `PWR_12` reader - "]
pub struct PWR_12_R(crate::FieldReader<u8, u8>);
impl PWR_12_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_12` writer - "]
pub struct PWR_12_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `PWR_13` reader - "]
pub struct PWR_13_R(crate::FieldReader<u8, u8>);
impl PWR_13_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_13` writer - "]
pub struct PWR_13_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `PWR_14` reader - "]
pub struct PWR_14_R(crate::FieldReader<u8, u8>);
impl PWR_14_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_14` writer - "]
pub struct PWR_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `PWR_15` reader - "]
pub struct PWR_15_R(crate::FieldReader<u8, u8>);
impl PWR_15_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWR_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWR_15` writer - "]
pub struct PWR_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_15_W<'a> {
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
    pub fn pwr_0(&self) -> PWR_0_R {
        PWR_0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pwr_1(&self) -> PWR_1_R {
        PWR_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pwr_2(&self) -> PWR_2_R {
        PWR_2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pwr_3(&self) -> PWR_3_R {
        PWR_3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pwr_4(&self) -> PWR_4_R {
        PWR_4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pwr_5(&self) -> PWR_5_R {
        PWR_5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pwr_6(&self) -> PWR_6_R {
        PWR_6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pwr_7(&self) -> PWR_7_R {
        PWR_7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pwr_8(&self) -> PWR_8_R {
        PWR_8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pwr_9(&self) -> PWR_9_R {
        PWR_9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pwr_10(&self) -> PWR_10_R {
        PWR_10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pwr_11(&self) -> PWR_11_R {
        PWR_11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pwr_12(&self) -> PWR_12_R {
        PWR_12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwr_13(&self) -> PWR_13_R {
        PWR_13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pwr_14(&self) -> PWR_14_R {
        PWR_14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pwr_15(&self) -> PWR_15_R {
        PWR_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn pwr_0(&mut self) -> PWR_0_W {
        PWR_0_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn pwr_1(&mut self) -> PWR_1_W {
        PWR_1_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn pwr_2(&mut self) -> PWR_2_W {
        PWR_2_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pwr_3(&mut self) -> PWR_3_W {
        PWR_3_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn pwr_4(&mut self) -> PWR_4_W {
        PWR_4_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn pwr_5(&mut self) -> PWR_5_W {
        PWR_5_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn pwr_6(&mut self) -> PWR_6_W {
        PWR_6_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn pwr_7(&mut self) -> PWR_7_W {
        PWR_7_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn pwr_8(&mut self) -> PWR_8_W {
        PWR_8_W { w: self }
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn pwr_9(&mut self) -> PWR_9_W {
        PWR_9_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pwr_10(&mut self) -> PWR_10_W {
        PWR_10_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn pwr_11(&mut self) -> PWR_11_W {
        PWR_11_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn pwr_12(&mut self) -> PWR_12_W {
        PWR_12_W { w: self }
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn pwr_13(&mut self) -> PWR_13_W {
        PWR_13_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn pwr_14(&mut self) -> PWR_14_W {
        PWR_14_W { w: self }
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn pwr_15(&mut self) -> PWR_15_W {
        PWR_15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PORT Power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr](index.html) module"]
pub struct PWR_SPEC;
impl crate::RegisterSpec for PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr::R](R) reader structure"]
impl crate::Readable for PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr::W](W) writer structure"]
impl crate::Writable for PWR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR to value 0"]
impl crate::Resettable for PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
