#[doc = "Register `PER_CLOCK` reader"]
pub struct R(crate::R<PER_CLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PER_CLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PER_CLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PER_CLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PER_CLOCK` writer"]
pub struct W(crate::W<PER_CLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PER_CLOCK_SPEC>;
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
impl From<crate::W<PER_CLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PER_CLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLK_CAN1` reader - "]
pub struct PCLK_CAN1_R(crate::FieldReader<bool, bool>);
impl PCLK_CAN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_CAN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_CAN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_CAN1` writer - "]
pub struct PCLK_CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_CAN1_W<'a> {
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
#[doc = "Field `PCLK_CAN2` reader - "]
pub struct PCLK_CAN2_R(crate::FieldReader<bool, bool>);
impl PCLK_CAN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_CAN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_CAN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_CAN2` writer - "]
pub struct PCLK_CAN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_CAN2_W<'a> {
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
#[doc = "Field `PCLK_USB` reader - "]
pub struct PCLK_USB_R(crate::FieldReader<bool, bool>);
impl PCLK_USB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_USB` writer - "]
pub struct PCLK_USB_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_USB_W<'a> {
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
#[doc = "Field `PCLK_EEPROM` reader - "]
pub struct PCLK_EEPROM_R(crate::FieldReader<bool, bool>);
impl PCLK_EEPROM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_EEPROM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_EEPROM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_EEPROM` writer - "]
pub struct PCLK_EEPROM_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_EEPROM_W<'a> {
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
#[doc = "Field `PCLK_RST_CLK` reader - "]
pub struct PCLK_RST_CLK_R(crate::FieldReader<bool, bool>);
impl PCLK_RST_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_RST_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_RST_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_RST_CLK` writer - "]
pub struct PCLK_RST_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_RST_CLK_W<'a> {
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
#[doc = "Field `PCLK_DMA` reader - "]
pub struct PCLK_DMA_R(crate::FieldReader<bool, bool>);
impl PCLK_DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_DMA` writer - "]
pub struct PCLK_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PCLK_UART1` reader - "]
pub struct PCLK_UART1_R(crate::FieldReader<bool, bool>);
impl PCLK_UART1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_UART1` writer - "]
pub struct PCLK_UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PCLK_UART2` reader - "]
pub struct PCLK_UART2_R(crate::FieldReader<bool, bool>);
impl PCLK_UART2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_UART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_UART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_UART2` writer - "]
pub struct PCLK_UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_UART2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PCLK_SPI1` reader - "]
pub struct PCLK_SPI1_R(crate::FieldReader<bool, bool>);
impl PCLK_SPI1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_SPI1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_SPI1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_SPI1` writer - "]
pub struct PCLK_SPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SPI1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PCLK_I2C` reader - "]
pub struct PCLK_I2C_R(crate::FieldReader<bool, bool>);
impl PCLK_I2C_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_I2C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_I2C` writer - "]
pub struct PCLK_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_I2C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `PCLK_POWER` reader - "]
pub struct PCLK_POWER_R(crate::FieldReader<bool, bool>);
impl PCLK_POWER_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_POWER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_POWER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_POWER` writer - "]
pub struct PCLK_POWER_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_POWER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `PCLK_WWDG` reader - "]
pub struct PCLK_WWDG_R(crate::FieldReader<bool, bool>);
impl PCLK_WWDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_WWDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_WWDG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_WWDG` writer - "]
pub struct PCLK_WWDG_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_WWDG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `PCLK_IWDG` reader - "]
pub struct PCLK_IWDG_R(crate::FieldReader<bool, bool>);
impl PCLK_IWDG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_IWDG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_IWDG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_IWDG` writer - "]
pub struct PCLK_IWDG_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_IWDG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `PCLK_TIMER1` reader - "]
pub struct PCLK_TIMER1_R(crate::FieldReader<bool, bool>);
impl PCLK_TIMER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_TIMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_TIMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_TIMER1` writer - "]
pub struct PCLK_TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `PCLK_TIMER2` reader - "]
pub struct PCLK_TIMER2_R(crate::FieldReader<bool, bool>);
impl PCLK_TIMER2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_TIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_TIMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_TIMER2` writer - "]
pub struct PCLK_TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `PCLK_TIMER3` reader - "]
pub struct PCLK_TIMER3_R(crate::FieldReader<bool, bool>);
impl PCLK_TIMER3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_TIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_TIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_TIMER3` writer - "]
pub struct PCLK_TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_TIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PCLK_ADC` reader - "]
pub struct PCLK_ADC_R(crate::FieldReader<bool, bool>);
impl PCLK_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_ADC` writer - "]
pub struct PCLK_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PCLK_DAC` reader - "]
pub struct PCLK_DAC_R(crate::FieldReader<bool, bool>);
impl PCLK_DAC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_DAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_DAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_DAC` writer - "]
pub struct PCLK_DAC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_DAC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PCLK_COMP` reader - "]
pub struct PCLK_COMP_R(crate::FieldReader<bool, bool>);
impl PCLK_COMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_COMP` writer - "]
pub struct PCLK_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PCLK_SPI2` reader - "]
pub struct PCLK_SPI2_R(crate::FieldReader<bool, bool>);
impl PCLK_SPI2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_SPI2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_SPI2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_SPI2` writer - "]
pub struct PCLK_SPI2_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_SPI2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PCLK_PORTA` reader - "]
pub struct PCLK_PORTA_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTA` writer - "]
pub struct PCLK_PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PCLK_PORTB` reader - "]
pub struct PCLK_PORTB_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTB` writer - "]
pub struct PCLK_PORTB_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTB_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PCLK_PORTC` reader - "]
pub struct PCLK_PORTC_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTC` writer - "]
pub struct PCLK_PORTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PCLK_PORTD` reader - "]
pub struct PCLK_PORTD_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTD` writer - "]
pub struct PCLK_PORTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PCLK_PORTE` reader - "]
pub struct PCLK_PORTE_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTE` writer - "]
pub struct PCLK_PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PCLK_BKP` reader - "]
pub struct PCLK_BKP_R(crate::FieldReader<bool, bool>);
impl PCLK_BKP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_BKP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_BKP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_BKP` writer - "]
pub struct PCLK_BKP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_BKP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PCLK_PORTF` reader - "]
pub struct PCLK_PORTF_R(crate::FieldReader<bool, bool>);
impl PCLK_PORTF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_PORTF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_PORTF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_PORTF` writer - "]
pub struct PCLK_PORTF_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_PORTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PCLK_EXT_BUS` reader - "]
pub struct PCLK_EXT_BUS_R(crate::FieldReader<bool, bool>);
impl PCLK_EXT_BUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCLK_EXT_BUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCLK_EXT_BUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCLK_EXT_BUS` writer - "]
pub struct PCLK_EXT_BUS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCLK_EXT_BUS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pclk_can1(&self) -> PCLK_CAN1_R {
        PCLK_CAN1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pclk_can2(&self) -> PCLK_CAN2_R {
        PCLK_CAN2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pclk_usb(&self) -> PCLK_USB_R {
        PCLK_USB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pclk_eeprom(&self) -> PCLK_EEPROM_R {
        PCLK_EEPROM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pclk_rst_clk(&self) -> PCLK_RST_CLK_R {
        PCLK_RST_CLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pclk_dma(&self) -> PCLK_DMA_R {
        PCLK_DMA_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pclk_uart1(&self) -> PCLK_UART1_R {
        PCLK_UART1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pclk_uart2(&self) -> PCLK_UART2_R {
        PCLK_UART2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pclk_spi1(&self) -> PCLK_SPI1_R {
        PCLK_SPI1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pclk_i2c(&self) -> PCLK_I2C_R {
        PCLK_I2C_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pclk_power(&self) -> PCLK_POWER_R {
        PCLK_POWER_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pclk_wwdg(&self) -> PCLK_WWDG_R {
        PCLK_WWDG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pclk_iwdg(&self) -> PCLK_IWDG_R {
        PCLK_IWDG_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pclk_timer1(&self) -> PCLK_TIMER1_R {
        PCLK_TIMER1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pclk_timer2(&self) -> PCLK_TIMER2_R {
        PCLK_TIMER2_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pclk_timer3(&self) -> PCLK_TIMER3_R {
        PCLK_TIMER3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pclk_adc(&self) -> PCLK_ADC_R {
        PCLK_ADC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pclk_dac(&self) -> PCLK_DAC_R {
        PCLK_DAC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pclk_comp(&self) -> PCLK_COMP_R {
        PCLK_COMP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pclk_spi2(&self) -> PCLK_SPI2_R {
        PCLK_SPI2_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pclk_porta(&self) -> PCLK_PORTA_R {
        PCLK_PORTA_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pclk_portb(&self) -> PCLK_PORTB_R {
        PCLK_PORTB_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pclk_portc(&self) -> PCLK_PORTC_R {
        PCLK_PORTC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pclk_portd(&self) -> PCLK_PORTD_R {
        PCLK_PORTD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pclk_porte(&self) -> PCLK_PORTE_R {
        PCLK_PORTE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pclk_bkp(&self) -> PCLK_BKP_R {
        PCLK_BKP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pclk_portf(&self) -> PCLK_PORTF_R {
        PCLK_PORTF_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pclk_ext_bus(&self) -> PCLK_EXT_BUS_R {
        PCLK_EXT_BUS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pclk_can1(&mut self) -> PCLK_CAN1_W {
        PCLK_CAN1_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pclk_can2(&mut self) -> PCLK_CAN2_W {
        PCLK_CAN2_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pclk_usb(&mut self) -> PCLK_USB_W {
        PCLK_USB_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pclk_eeprom(&mut self) -> PCLK_EEPROM_W {
        PCLK_EEPROM_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pclk_rst_clk(&mut self) -> PCLK_RST_CLK_W {
        PCLK_RST_CLK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pclk_dma(&mut self) -> PCLK_DMA_W {
        PCLK_DMA_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pclk_uart1(&mut self) -> PCLK_UART1_W {
        PCLK_UART1_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pclk_uart2(&mut self) -> PCLK_UART2_W {
        PCLK_UART2_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pclk_spi1(&mut self) -> PCLK_SPI1_W {
        PCLK_SPI1_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pclk_i2c(&mut self) -> PCLK_I2C_W {
        PCLK_I2C_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pclk_power(&mut self) -> PCLK_POWER_W {
        PCLK_POWER_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pclk_wwdg(&mut self) -> PCLK_WWDG_W {
        PCLK_WWDG_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pclk_iwdg(&mut self) -> PCLK_IWDG_W {
        PCLK_IWDG_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pclk_timer1(&mut self) -> PCLK_TIMER1_W {
        PCLK_TIMER1_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pclk_timer2(&mut self) -> PCLK_TIMER2_W {
        PCLK_TIMER2_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pclk_timer3(&mut self) -> PCLK_TIMER3_W {
        PCLK_TIMER3_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pclk_adc(&mut self) -> PCLK_ADC_W {
        PCLK_ADC_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pclk_dac(&mut self) -> PCLK_DAC_W {
        PCLK_DAC_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pclk_comp(&mut self) -> PCLK_COMP_W {
        PCLK_COMP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pclk_spi2(&mut self) -> PCLK_SPI2_W {
        PCLK_SPI2_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pclk_porta(&mut self) -> PCLK_PORTA_W {
        PCLK_PORTA_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pclk_portb(&mut self) -> PCLK_PORTB_W {
        PCLK_PORTB_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pclk_portc(&mut self) -> PCLK_PORTC_W {
        PCLK_PORTC_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pclk_portd(&mut self) -> PCLK_PORTD_W {
        PCLK_PORTD_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pclk_porte(&mut self) -> PCLK_PORTE_W {
        PCLK_PORTE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn pclk_bkp(&mut self) -> PCLK_BKP_W {
        PCLK_BKP_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pclk_portf(&mut self) -> PCLK_PORTF_W {
        PCLK_PORTF_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pclk_ext_bus(&mut self) -> PCLK_EXT_BUS_W {
        PCLK_EXT_BUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [per_clock](index.html) module"]
pub struct PER_CLOCK_SPEC;
impl crate::RegisterSpec for PER_CLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [per_clock::R](R) reader structure"]
impl crate::Readable for PER_CLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [per_clock::W](W) writer structure"]
impl crate::Writable for PER_CLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PER_CLOCK to value 0x10"]
impl crate::Resettable for PER_CLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
