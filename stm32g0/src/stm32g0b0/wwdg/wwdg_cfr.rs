#[doc = "Reader of register WWDG_CFR"]
pub type R = crate::R<u32, super::WWDG_CFR>;
#[doc = "Writer for register WWDG_CFR"]
pub type W = crate::W<u32, super::WWDG_CFR>;
#[doc = "Register WWDG_CFR `reset()`'s with value 0x7f"]
impl crate::ResetValue for super::WWDG_CFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x7f
    }
}
#[doc = "Reader of field `W`"]
pub type W_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `W`"]
pub struct W_W<'a> {
    w: &'a mut W,
}
impl<'a> W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `EWI`"]
pub type EWI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EWI`"]
pub struct EWI_W<'a> {
    w: &'a mut W,
}
impl<'a> EWI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Timer base The timebase of the prescaler can be modified as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDGTB_A {
    #[doc = "0: CK Counter Clock (PCLK div 4096) div 1"]
    B_0X0 = 0,
    #[doc = "1: CK Counter Clock (PCLK div 4096) div 2"]
    B_0X1 = 1,
    #[doc = "2: CK Counter Clock (PCLK div 4096) div 4"]
    B_0X2 = 2,
    #[doc = "3: CK Counter Clock (PCLK div 4096) div 8"]
    B_0X3 = 3,
    #[doc = "4: CK Counter Clock (PCLK div 4096) div 16"]
    B_0X4 = 4,
    #[doc = "5: CK Counter Clock (PCLK div 4096) div 32"]
    B_0X5 = 5,
    #[doc = "6: CK Counter Clock (PCLK div 4096) div 64"]
    B_0X6 = 6,
    #[doc = "7: CK Counter Clock (PCLK div 4096) div 128"]
    B_0X7 = 7,
}
impl From<WDGTB_A> for u8 {
    #[inline(always)]
    fn from(variant: WDGTB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDGTB`"]
pub type WDGTB_R = crate::R<u8, WDGTB_A>;
impl WDGTB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDGTB_A {
        match self.bits {
            0 => WDGTB_A::B_0X0,
            1 => WDGTB_A::B_0X1,
            2 => WDGTB_A::B_0X2,
            3 => WDGTB_A::B_0X3,
            4 => WDGTB_A::B_0X4,
            5 => WDGTB_A::B_0X5,
            6 => WDGTB_A::B_0X6,
            7 => WDGTB_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDGTB_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDGTB_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == WDGTB_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == WDGTB_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == WDGTB_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == WDGTB_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == WDGTB_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == WDGTB_A::B_0X7
    }
}
#[doc = "Write proxy for field `WDGTB`"]
pub struct WDGTB_W<'a> {
    w: &'a mut W,
}
impl<'a> WDGTB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDGTB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 1"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X0)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X1)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X2)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X3)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 16"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X4)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 32"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X5)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 64"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X6)
    }
    #[doc = "CK Counter Clock (PCLK div 4096) div 128"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(WDGTB_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn w(&self) -> W_R {
        W_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn ewi(&self) -> EWI_R {
        EWI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WDGTB_R {
        WDGTB_R::new(((self.bits >> 11) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared with the down-counter."]
    #[inline(always)]
    pub fn w(&mut self) -> W_W {
        W_W { w: self }
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn ewi(&mut self) -> EWI_W {
        EWI_W { w: self }
    }
    #[doc = "Bits 11:13 - Timer base The timebase of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WDGTB_W {
        WDGTB_W { w: self }
    }
}
