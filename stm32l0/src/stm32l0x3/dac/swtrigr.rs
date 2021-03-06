#[doc = "Writer for register SWTRIGR"]
pub type W = crate::W<u32, super::SWTRIGR>;
#[doc = "Register SWTRIGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SWTRIGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DAC channel1 software trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRIG1_AW {
    #[doc = "0: DAC channel X software trigger disabled"]
    DISABLED = 0,
    #[doc = "1: DAC channel X software trigger enabled"]
    ENABLED = 1,
}
impl From<SWTRIG1_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRIG1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWTRIG1`"]
pub struct SWTRIG1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRIG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRIG1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DAC channel X software trigger disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::DISABLED)
    }
    #[doc = "DAC channel X software trigger enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SWTRIG1_AW::ENABLED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger"]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> SWTRIG1_W {
        SWTRIG1_W { w: self }
    }
}
