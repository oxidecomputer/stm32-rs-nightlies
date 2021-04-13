#[doc = "Reader of register AF1"]
pub type R = crate::R<u32, super::AF1>;
#[doc = "Writer for register AF1"]
pub type W = crate::W<u32, super::AF1>;
#[doc = "Register AF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ETR source selection These bits select the ETR input source. Others: Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRSEL_A {
    #[doc = "0: ETR legacy mode"]
    B_0X0 = 0,
    #[doc = "1: COMP1"]
    B_0X1 = 1,
    #[doc = "2: COMP2"]
    B_0X2 = 2,
    #[doc = "3: LSE"]
    B_0X3 = 3,
}
impl From<ETRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETRSEL`"]
pub type ETRSEL_R = crate::R<u8, ETRSEL_A>;
impl ETRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETRSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETRSEL_A::B_0X0),
            1 => Val(ETRSEL_A::B_0X1),
            2 => Val(ETRSEL_A::B_0X2),
            3 => Val(ETRSEL_A::B_0X3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ETRSEL_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ETRSEL_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == ETRSEL_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == ETRSEL_A::B_0X3
    }
}
#[doc = "Write proxy for field `ETRSEL`"]
pub struct ETRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X0)
    }
    #[doc = "COMP1"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X1)
    }
    #[doc = "COMP2"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X2)
    }
    #[doc = "LSE"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(ETRSEL_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 14)) | (((value as u32) & 0x0f) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:17 - ETR source selection These bits select the ETR input source. Others: Reserved"]
    #[inline(always)]
    pub fn etrsel(&mut self) -> ETRSEL_W {
        ETRSEL_W { w: self }
    }
}
