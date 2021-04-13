#[doc = "Reader of register WWDG_CR"]
pub type R = crate::R<u32, super::WWDG_CR>;
#[doc = "Writer for register WWDG_CR"]
pub type W = crate::W<u32, super::WWDG_CR>;
#[doc = "Register WWDG_CR `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::WWDG_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
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
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGAÂ =Â 1, the watchdog can generate a reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDGA_A {
    #[doc = "0: Watchdog disabled"]
    B_0X0 = 0,
    #[doc = "1: Watchdog enabled"]
    B_0X1 = 1,
}
impl From<WDGA_A> for bool {
    #[inline(always)]
    fn from(variant: WDGA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WDGA`"]
pub type WDGA_R = crate::R<bool, WDGA_A>;
impl WDGA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDGA_A {
        match self.bits {
            false => WDGA_A::B_0X0,
            true => WDGA_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGA_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGA_A::B_0X1
    }
}
#[doc = "Write proxy for field `WDGA`"]
pub struct WDGA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDGA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Watchdog disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDGA_A::B_0X0)
    }
    #[doc = "Watchdog enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDGA_A::B_0X1)
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
impl R {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGAÂ =Â 1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn wdga(&self) -> WDGA_R {
        WDGA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit counter (MSB to LSB) These bits contain the value of the watchdog counter, decremented every (4096 x 2WDGTB\\[1:0\\]) PCLK cycles. A reset is produced when it is decremented from 0x40 to 0x3F (T6 becomes cleared)."]
    #[inline(always)]
    pub fn t(&mut self) -> T_W {
        T_W { w: self }
    }
    #[doc = "Bit 7 - Activation bit This bit is set by software and only cleared by hardware after a reset. When WDGAÂ =Â 1, the watchdog can generate a reset."]
    #[inline(always)]
    pub fn wdga(&mut self) -> WDGA_W {
        WDGA_W { w: self }
    }
}
