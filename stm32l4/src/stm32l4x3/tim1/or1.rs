#[doc = "Reader of register OR1"]
pub type R = crate::R<u32, super::OR1>;
#[doc = "Writer for register OR1"]
pub type W = crate::W<u32, super::OR1>;
#[doc = "Register OR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ETR_ADC1_RMP`"]
pub type ETR_ADC1_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETR_ADC1_RMP`"]
pub struct ETR_ADC1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_ADC1_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ETR_ADC3_RMP`"]
pub type ETR_ADC3_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ETR_ADC3_RMP`"]
pub struct ETR_ADC3_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ETR_ADC3_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `TI1_RMP`"]
pub type TI1_RMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TI1_RMP`"]
pub struct TI1_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1_RMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External trigger remap on ADC1 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc1_rmp(&self) -> ETR_ADC1_RMP_R {
        ETR_ADC1_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&self) -> ETR_ADC3_RMP_R {
        ETR_ADC3_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External trigger remap on ADC1 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc1_rmp(&mut self) -> ETR_ADC1_RMP_W {
        ETR_ADC1_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog"]
    #[inline(always)]
    pub fn etr_adc3_rmp(&mut self) -> ETR_ADC3_RMP_W {
        ETR_ADC3_RMP_W { w: self }
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W {
        TI1_RMP_W { w: self }
    }
}
