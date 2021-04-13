#[doc = "Reader of register PRESC"]
pub type R = crate::R<u32, super::PRESC>;
#[doc = "Writer for register PRESC"]
pub type W = crate::W<u32, super::PRESC>;
#[doc = "Register PRESC `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: input clock not divided"]
    B_0X0 = 0,
    #[doc = "1: input clock divided by 2"]
    B_0X1 = 1,
    #[doc = "2: input clock divided by 4"]
    B_0X2 = 2,
    #[doc = "3: input clock divided by 6"]
    B_0X3 = 3,
    #[doc = "4: input clock divided by 8"]
    B_0X4 = 4,
    #[doc = "5: input clock divided by 10"]
    B_0X5 = 5,
    #[doc = "6: input clock divided by 12"]
    B_0X6 = 6,
    #[doc = "7: input clock divided by 16"]
    B_0X7 = 7,
    #[doc = "8: input clock divided by 32"]
    B_0X8 = 8,
    #[doc = "9: input clock divided by 64"]
    B_0X9 = 9,
    #[doc = "10: input clock divided by 128"]
    B_0XA = 10,
    #[doc = "11: input clock divided by 256"]
    B_0XB = 11,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALER`"]
pub type PRESCALER_R = crate::R<u8, PRESCALER_A>;
impl PRESCALER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCALER_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCALER_A::B_0X0),
            1 => Val(PRESCALER_A::B_0X1),
            2 => Val(PRESCALER_A::B_0X2),
            3 => Val(PRESCALER_A::B_0X3),
            4 => Val(PRESCALER_A::B_0X4),
            5 => Val(PRESCALER_A::B_0X5),
            6 => Val(PRESCALER_A::B_0X6),
            7 => Val(PRESCALER_A::B_0X7),
            8 => Val(PRESCALER_A::B_0X8),
            9 => Val(PRESCALER_A::B_0X9),
            10 => Val(PRESCALER_A::B_0XA),
            11 => Val(PRESCALER_A::B_0XB),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PRESCALER_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PRESCALER_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PRESCALER_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PRESCALER_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == PRESCALER_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == PRESCALER_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == PRESCALER_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == PRESCALER_A::B_0X7
    }
    #[doc = "Checks if the value of the field is `B_0X8`"]
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == PRESCALER_A::B_0X8
    }
    #[doc = "Checks if the value of the field is `B_0X9`"]
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == PRESCALER_A::B_0X9
    }
    #[doc = "Checks if the value of the field is `B_0XA`"]
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == PRESCALER_A::B_0XA
    }
    #[doc = "Checks if the value of the field is `B_0XB`"]
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == PRESCALER_A::B_0XB
    }
}
#[doc = "Write proxy for field `PRESCALER`"]
pub struct PRESCALER_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "input clock not divided"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X0)
    }
    #[doc = "input clock divided by 2"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X1)
    }
    #[doc = "input clock divided by 4"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X2)
    }
    #[doc = "input clock divided by 6"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X3)
    }
    #[doc = "input clock divided by 8"]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X4)
    }
    #[doc = "input clock divided by 10"]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X5)
    }
    #[doc = "input clock divided by 12"]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X6)
    }
    #[doc = "input clock divided by 16"]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X7)
    }
    #[doc = "input clock divided by 32"]
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X8)
    }
    #[doc = "input clock divided by 64"]
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0X9)
    }
    #[doc = "input clock divided by 128"]
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0XA)
    }
    #[doc = "input clock divided by 256"]
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut W {
        self.variant(PRESCALER_A::B_0XB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock prescaler The USART input clock can be divided by a prescaler factor: Remaining combinations: Reserved Note: When PRESCALER is programmed with a value different of the allowed ones, programmed prescaler value is 1011 i.e. input clock divided by 256."]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PRESCALER_W {
        PRESCALER_W { w: self }
    }
}
