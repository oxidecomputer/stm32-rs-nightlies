#[doc = "Reader of register TIM1_OR1"]
pub type R = crate::R<u32, super::TIM1_OR1>;
#[doc = "Writer for register TIM1_OR1"]
pub type W = crate::W<u32, super::TIM1_OR1>;
#[doc = "Register TIM1_OR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1_OR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Ocref_clr source selection This bit selects the ocref_clr input source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OCREF_CLR_A {
    #[doc = "0: COMP1 output is connected to the OCREF_CLR input"]
    B_0X0 = 0,
    #[doc = "1: COMP2 output is connected to the OCREF_CLR input"]
    B_0X1 = 1,
}
impl From<OCREF_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: OCREF_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OCREF_CLR`"]
pub type OCREF_CLR_R = crate::R<bool, OCREF_CLR_A>;
impl OCREF_CLR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OCREF_CLR_A {
        match self.bits {
            false => OCREF_CLR_A::B_0X0,
            true => OCREF_CLR_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OCREF_CLR_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OCREF_CLR_A::B_0X1
    }
}
#[doc = "Write proxy for field `OCREF_CLR`"]
pub struct OCREF_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OCREF_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCREF_CLR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP1 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OCREF_CLR_A::B_0X0)
    }
    #[doc = "COMP2 output is connected to the OCREF_CLR input"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OCREF_CLR_A::B_0X1)
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
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection This bit selects the ocref_clr input source."]
    #[inline(always)]
    pub fn ocref_clr(&mut self) -> OCREF_CLR_W {
        OCREF_CLR_W { w: self }
    }
}
