#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Activation bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDGA_AW {
    #[doc = "0: Watchdog disabled"]
    DISABLED = 0,
    #[doc = "1: Watchdog enabled"]
    ENABLED = 1,
}
impl From<WDGA_AW> for bool {
    #[inline(always)]
    fn from(variant: WDGA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `WDGA`"]
pub struct WDGA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDGA_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDGA_AW::DISABLED)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WDGA_AW::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `T`"]
pub type T_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `T`"]
pub struct T_W<'a> {
    w: &'a mut W,
}
impl<'a> T_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Activation bit"]
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W {
        WDGA_W { w: self }
    }
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB)"]
    #[inline(always)]
    pub fn t(&mut self) -> T_W {
        T_W { w: self }
    }
}
