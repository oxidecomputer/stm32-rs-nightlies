#[doc = "Reader of register ADC_AWD1TR"]
pub type R = crate::R<u32, super::ADC_AWD1TR>;
#[doc = "Writer for register ADC_AWD1TR"]
pub type W = crate::W<u32, super::ADC_AWD1TR>;
#[doc = "Register ADC_AWD1TR `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::ADC_AWD1TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `LT1`"]
pub type LT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LT1`"]
pub struct LT1_W<'a> {
    w: &'a mut W,
}
impl<'a> LT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `HT1`"]
pub type HT1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HT1`"]
pub struct HT1_W<'a> {
    w: &'a mut W,
}
impl<'a> HT1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn lt1(&self) -> LT1_R {
        LT1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn ht1(&self) -> HT1_R {
        HT1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 1 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn lt1(&mut self) -> LT1_W {
        LT1_W { w: self }
    }
    #[doc = "Bits 16:27 - Analog watchdog 1 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn ht1(&mut self) -> HT1_W {
        HT1_W { w: self }
    }
}
