#[doc = "Reader of register ADC_AWD2TR"]
pub type R = crate::R<u32, super::ADC_AWD2TR>;
#[doc = "Writer for register ADC_AWD2TR"]
pub type W = crate::W<u32, super::ADC_AWD2TR>;
#[doc = "Register ADC_AWD2TR `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::ADC_AWD2TR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `LT2`"]
pub type LT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LT2`"]
pub struct LT2_W<'a> {
    w: &'a mut W,
}
impl<'a> LT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `HT2`"]
pub type HT2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HT2`"]
pub struct HT2_W<'a> {
    w: &'a mut W,
}
impl<'a> HT2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Analog watchdog 2 lower threshold These bits are written by software to define the lower threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn lt2(&mut self) -> LT2_W {
        LT2_W { w: self }
    }
    #[doc = "Bits 16:27 - Analog watchdog 2 higher threshold These bits are written by software to define the higher threshold for the analog watchdog. Refer to ADC_AWDxTR) on pageÂ 395."]
    #[inline(always)]
    pub fn ht2(&mut self) -> HT2_W {
        HT2_W { w: self }
    }
}
