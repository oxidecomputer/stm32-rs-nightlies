#[doc = "Reader of register CCMR1_Input"]
pub type R = crate::R<u32, super::CCMR1_INPUT>;
#[doc = "Writer for register CCMR1_Input"]
pub type W = crate::W<u32, super::CCMR1_INPUT>;
#[doc = "Register CCMR1_Input `reset()`'s with value 0"]
impl crate::ResetValue for super::CCMR1_INPUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "0: CC1 channel is configured as output"]
    B_0X0 = 0,
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    B_0X1 = 1,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CC1S`"]
pub type CC1S_R = crate::R<u8, CC1S_A>;
impl CC1S_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC1S_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC1S_A::B_0X0),
            1 => Val(CC1S_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1S_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1S_A::B_0X1
    }
}
#[doc = "Write proxy for field `CC1S`"]
pub struct CC1S_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1S_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1S_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "CC1 channel is configured as output"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X0)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CC1S_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â\u{80}\u{99}0â\u{80}\u{99} (TIMx_CCER register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1PSC_A {
    #[doc = "0: no prescaler, capture is done each time an edge is detected on the capture input"]
    B_0X0 = 0,
    #[doc = "1: capture is done once every 2 events"]
    B_0X1 = 1,
    #[doc = "2: capture is done once every 4 events"]
    B_0X2 = 2,
    #[doc = "3: capture is done once every 8 events"]
    B_0X3 = 3,
}
impl From<IC1PSC_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1PSC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IC1PSC`"]
pub type IC1PSC_R = crate::R<u8, IC1PSC_A>;
impl IC1PSC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC1PSC_A {
        match self.bits {
            0 => IC1PSC_A::B_0X0,
            1 => IC1PSC_A::B_0X1,
            2 => IC1PSC_A::B_0X2,
            3 => IC1PSC_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IC1PSC_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IC1PSC_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IC1PSC_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == IC1PSC_A::B_0X3
    }
}
#[doc = "Write proxy for field `IC1PSC`"]
pub struct IC1PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1PSC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC1PSC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "no prescaler, capture is done each time an edge is detected on the capture input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X0)
    }
    #[doc = "capture is done once every 2 events"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X1)
    }
    #[doc = "capture is done once every 4 events"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X2)
    }
    #[doc = "capture is done once every 8 events"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IC1PSC_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IC1F_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    B_0X0 = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    B_0X1 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    B_0X2 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    B_0X3 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    B_0X4 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    B_0X5 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    B_0X6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    B_0X7 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    B_0X8 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    B_0X9 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    B_0XA = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    B_0XB = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    B_0XC = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    B_0XD = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    B_0XE = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    B_0XF = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IC1F`"]
pub type IC1F_R = crate::R<u8, IC1F_A>;
impl IC1F_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::B_0X0,
            1 => IC1F_A::B_0X1,
            2 => IC1F_A::B_0X2,
            3 => IC1F_A::B_0X3,
            4 => IC1F_A::B_0X4,
            5 => IC1F_A::B_0X5,
            6 => IC1F_A::B_0X6,
            7 => IC1F_A::B_0X7,
            8 => IC1F_A::B_0X8,
            9 => IC1F_A::B_0X9,
            10 => IC1F_A::B_0XA,
            11 => IC1F_A::B_0XB,
            12 => IC1F_A::B_0XC,
            13 => IC1F_A::B_0XD,
            14 => IC1F_A::B_0XE,
            15 => IC1F_A::B_0XF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IC1F_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IC1F_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == IC1F_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == IC1F_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == IC1F_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == IC1F_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == IC1F_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == IC1F_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == IC1F_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == IC1F_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == IC1F_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == IC1F_A::B_0XB
    }
    #[doc = "Checks if the value of the field is `B_0XC`"]
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == IC1F_A::B_0XC
    }
    #[doc = "Checks if the value of the field is `B_0XD`"]
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == IC1F_A::B_0XD
    }
    #[doc = "Checks if the value of the field is `B_0XE`"]
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == IC1F_A::B_0XE
    }
    #[doc = "Checks if the value of the field is `B_0XF`"]
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == IC1F_A::B_0XF
    }
}
#[doc = "Write proxy for field `IC1F`"]
pub struct IC1F_W<'a> {
    w: &'a mut W,
}
impl<'a> IC1F_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IC1F_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X0)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X1)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X3)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X4)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X5)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X7)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(IC1F_A::B_0X9)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XA)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XB)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XC)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XD)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XE)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut W {
        self.variant(IC1F_A::B_0XF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â\u{80}\u{99}0â\u{80}\u{99} (TIMx_CCER register)."]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC1S bits are writable only when the channel is OFF (CC1E = 0 in TIMx_CCER)."]
    #[inline(always)]
    pub fn cc1s(&mut self) -> CC1S_W {
        CC1S_W { w: self }
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler This bit-field defines the ratio of the prescaler acting on CC1 input (IC1). The prescaler is reset as soon as CC1E=â\u{80}\u{99}0â\u{80}\u{99} (TIMx_CCER register)."]
    #[inline(always)]
    pub fn ic1psc(&mut self) -> IC1PSC_W {
        IC1PSC_W { w: self }
    }
    #[doc = "Bits 4:7 - Input capture 1 filter This bit-field defines the frequency used to sample TI1 input and the length of the digital filter applied to TI1. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn ic1f(&mut self) -> IC1F_W {
        IC1F_W { w: self }
    }
}
