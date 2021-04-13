#[doc = "Reader of register ADC_CALFACT"]
pub type R = crate::R<u32, super::ADC_CALFACT>;
#[doc = "Writer for register ADC_CALFACT"]
pub type W = crate::W<u32, super::ADC_CALFACT>;
#[doc = "Register ADC_CALFACT `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_CALFACT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALFACT`"]
pub type CALFACT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CALFACT`"]
pub struct CALFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,Â they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new calibration is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing). Refer to SQ8\\[3:0\\]
for a definition of channel selection."]
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration factor These bits are written by hardware or by software. Once a calibration is complete,Â they are updated by hardware with the calibration factors. Software can write these bits with a new calibration factor. If the new calibration factor is different from the current one stored into the analog ADC, it is then applied once a new calibration is launched. Just after a calibration is complete, DATA\\[6:0\\]
contains the calibration factor. Note: Software can write these bits only when ADEN=1 (ADC is enabled and no calibration is ongoing and no conversion is ongoing). Refer to SQ8\\[3:0\\]
for a definition of channel selection."]
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W {
        CALFACT_W { w: self }
    }
}
