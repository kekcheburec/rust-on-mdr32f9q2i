#[doc = "Register `REG_09` reader"]
pub struct R(crate::R<REG_09_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_09_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_09_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_09_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_09` writer"]
pub struct W(crate::W<REG_09_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_09_SPEC>;
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
impl From<crate::W<REG_09_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_09_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP_REG` reader - "]
pub struct BKP_REG_R(crate::FieldReader<u32, u32>);
impl BKP_REG_R {
    pub(crate) fn new(bits: u32) -> Self {
        BKP_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKP_REG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKP_REG` writer - "]
pub struct BKP_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> BKP_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bkp_reg(&self) -> BKP_REG_R {
        BKP_REG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bkp_reg(&mut self) -> BKP_REG_W {
        BKP_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Register 9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_09](index.html) module"]
pub struct REG_09_SPEC;
impl crate::RegisterSpec for REG_09_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_09::R](R) reader structure"]
impl crate::Readable for REG_09_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_09::W](W) writer structure"]
impl crate::Writable for REG_09_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG_09 to value 0"]
impl crate::Resettable for REG_09_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
