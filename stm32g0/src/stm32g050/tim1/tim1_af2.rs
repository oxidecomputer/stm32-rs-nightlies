#[doc = "Reader of register TIM1_AF2"]
pub type R = crate::R<u32, super::TIM1_AF2>;
#[doc = "Writer for register TIM1_AF2"]
pub type W = crate::W<u32, super::TIM1_AF2>;
#[doc = "Register TIM1_AF2 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::TIM1_AF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INE_A {
    #[doc = "0: BKIN2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input enabled"]
    B_0X1 = 1,
}
impl From<BK2INE_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK2INE`"]
pub type BK2INE_R = crate::R<bool, BK2INE_A>;
impl BK2INE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INE_A {
        match self.bits {
            false => BK2INE_A::B_0X0,
            true => BK2INE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BK2INE`"]
pub struct BK2INE_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BKIN2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2INE_A::B_0X0)
    }
    #[doc = "BKIN2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2INE_A::B_0X1)
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
#[doc = "BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BK2INP_A {
    #[doc = "0: BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    B_0X0 = 0,
    #[doc = "1: BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    B_0X1 = 1,
}
impl From<BK2INP_A> for bool {
    #[inline(always)]
    fn from(variant: BK2INP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BK2INP`"]
pub type BK2INP_R = crate::R<bool, BK2INP_A>;
impl BK2INP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BK2INP_A {
        match self.bits {
            false => BK2INP_A::B_0X0,
            true => BK2INP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BK2INP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BK2INP_A::B_0X1
    }
}
#[doc = "Write proxy for field `BK2INP`"]
pub struct BK2INP_W<'a> {
    w: &'a mut W,
}
impl<'a> BK2INP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BK2INP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BKIN2 input polarity is not inverted (active low if BK2P=0, active high if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BK2INP_A::B_0X0)
    }
    #[doc = "BKIN2 input polarity is inverted (active high if BK2P=0, active low if BK2P=1)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BK2INP_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&self) -> BK2INE_R {
        BK2INE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&self) -> BK2INP_R {
        BK2INP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable This bit enables the BKIN2 alternate function input for the timerâ\u{80}\u{99}s BRK2 input. BKIN2 input is 'ORedâ\u{80}\u{99} with the other BRK2 sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> BK2INE_W {
        BK2INE_W { w: self }
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity This bit selects the BKIN2 alternate function input sensitivity. It must be programmed together with the BK2P polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> BK2INP_W {
        BK2INP_W { w: self }
    }
}
