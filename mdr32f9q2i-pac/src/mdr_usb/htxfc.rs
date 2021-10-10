#[doc = "Register `HTXFC` reader"]
pub struct R(crate::R<HTXFC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTXFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTXFC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTXFC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HTXFC` writer"]
pub struct W(crate::W<HTXFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HTXFC_SPEC>;
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
impl From<crate::W<HTXFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HTXFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFOFORCEEMPTY` reader - "]
pub struct FIFOFORCEEMPTY_R(crate::FieldReader<bool, bool>);
impl FIFOFORCEEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOFORCEEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOFORCEEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOFORCEEMPTY` writer - "]
pub struct FIFOFORCEEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOFORCEEMPTY_W<'a> {
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
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoforceempty(&self) -> FIFOFORCEEMPTY_R {
        FIFOFORCEEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifoforceempty(&mut self) -> FIFOFORCEEMPTY_W {
        FIFOFORCEEMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htxfc](index.html) module"]
pub struct HTXFC_SPEC;
impl crate::RegisterSpec for HTXFC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htxfc::R](R) reader structure"]
impl crate::Readable for HTXFC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [htxfc::W](W) writer structure"]
impl crate::Writable for HTXFC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HTXFC to value 0"]
impl crate::Resettable for HTXFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
