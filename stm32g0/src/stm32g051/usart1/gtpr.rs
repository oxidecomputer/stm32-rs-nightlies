#[doc = "Reader of register GTPR"]
pub type R = crate::R<u32, super::GTPR>;
#[doc = "Writer for register GTPR"]
pub type W = crate::W<u32, super::GTPR>;
#[doc = "Register GTPR `reset()`'s with value 0"]
impl crate::ResetValue for super::GTPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSC_A {
    #[doc = "0: Reserved - do not program this value"]
    B_0X0 = 0,
    #[doc = "1: Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    B_0X1 = 1,
    #[doc = "2: Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    B_0X2 = 2,
    #[doc = "3: Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    B_0X3 = 3,
    #[doc = "31: Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    B_0X1F = 31,
}
impl From<PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u8, PSC_A>;
impl PSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PSC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PSC_A::B_0X0),
            1 => Val(PSC_A::B_0X1),
            2 => Val(PSC_A::B_0X2),
            3 => Val(PSC_A::B_0X3),
            31 => Val(PSC_A::B_0X1F),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PSC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PSC_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X1F`"]
    #[inline(always)]
    pub fn is_b_0x1f(&self) -> bool {
        *self == PSC_A::B_0X1F
    }
}
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Reserved - do not program this value"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PSC_A::B_0X0)
    }
    #[doc = "Divides the source clock by 1 (IrDA mode) / by 2 (Smarcard mode)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PSC_A::B_0X1)
    }
    #[doc = "Divides the source clock by 2 (IrDA mode) / by 4 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PSC_A::B_0X2)
    }
    #[doc = "Divides the source clock by 3 (IrDA mode) / by 6 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PSC_A::B_0X3)
    }
    #[doc = "Divides the source clock by 31 (IrDA mode) / by 62 (Smartcard mode)"]
    #[inline(always)]
    pub fn b_0x1f(self) -> &'a mut W {
        self.variant(PSC_A::B_0X1F)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `GT`"]
pub type GT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GT`"]
pub struct GT_W<'a> {
    w: &'a mut W,
}
impl<'a> GT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&self) -> GT_R {
        GT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler value In IrDA low-power and normal IrDA mode: PSC\\[7:0\\]
= IrDA Normal and Low-Power baud rate PSC\\[7:0\\]
is used to program the prescaler for dividing the USART source clock to achieve the low-power frequency: the source clock is divided by the value given in the register (8 significant bits): In Smartcard mode: PSC\\[4:0\\]Â =Â Prescaler value PSC\\[4:0\\]
is used to program the prescaler for dividing the USART source clock to provide the Smartcard clock. The value given in the register (5 significant bits) is multiplied by 2 to give the division factor of the source clock frequency: ... 0010Â 0000: Divides the source clock by 32 (IrDA mode) ... 1111Â 1111: Divides the source clock by 255 (IrDA mode) This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: Bits \\[7:5\\]
must be kept cleared if Smartcard mode is used. This bitfield is reserved and forced by hardware to '0â\u{80}\u{99} when the Smartcard and IrDA modes are not supported. Refer to ."]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
    #[doc = "Bits 8:15 - Guard time value This bitfield is used to program the Guard time value in terms of number of baud clock periods. This is used in Smartcard mode. The Transmission Complete flag is set after this guard time value. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn gt(&mut self) -> GT_W {
        GT_W { w: self }
    }
}
