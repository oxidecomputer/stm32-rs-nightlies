#[doc = "Reader of register TISEL"]
pub type R = crate::R<u32, super::TISEL>;
#[doc = "Writer for register TISEL"]
pub type W = crate::W<u32, super::TISEL>;
#[doc = "Register TISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI1SEL_A {
    #[doc = "0: TIM2_CH1 input"]
    B_0X0 = 0,
    #[doc = "1: COMP1 output"]
    B_0X1 = 1,
}
impl From<TI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TI1SEL`"]
pub type TI1SEL_R = crate::R<u8, TI1SEL_A>;
impl TI1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TI1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TI1SEL_A::B_0X0),
            1 => Val(TI1SEL_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1SEL_A::B_0X1
    }
}
#[doc = "Write proxy for field `TI1SEL`"]
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2_CH1 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TI1SEL_A::B_0X0)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TI1SEL_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TI2SEL_A {
    #[doc = "0: TIM2_CH2 input"]
    B_0X0 = 0,
    #[doc = "1: COMP2 output"]
    B_0X1 = 1,
}
impl From<TI2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TI2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TI2SEL`"]
pub type TI2SEL_R = crate::R<u8, TI2SEL_A>;
impl TI2SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TI2SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TI2SEL_A::B_0X0),
            1 => Val(TI2SEL_A::B_0X1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI2SEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI2SEL_A::B_0X1
    }
}
#[doc = "Write proxy for field `TI2SEL`"]
pub struct TI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TI2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "TIM2_CH2 input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(TI2SEL_A::B_0X0)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(TI2SEL_A::B_0X1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1\\[0\\]
to TI1\\[15\\]
input selection These bits select the TI1\\[0\\]
to TI1\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - TI2\\[0\\]
to TI2\\[15\\]
input selection These bits select the TI2\\[0\\]
to TI2\\[15\\]
input source. Others: Reserved"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W {
        TI2SEL_W { w: self }
    }
}
