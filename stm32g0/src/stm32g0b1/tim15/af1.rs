#[doc = "Reader of register AF1"]
pub type R = crate::R<u32, super::AF1>;
#[doc = "Writer for register AF1"]
pub type W = crate::W<u32, super::AF1>;
#[doc = "Register AF1 `reset()`'s with value 0x01"]
impl crate::ResetValue for super::AF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKINE_A {
    #[doc = "0: BKIN input disabled"]
    B_0X0 = 0,
    #[doc = "1: BKIN input enabled"]
    B_0X1 = 1,
}
impl From<BKINE_A> for bool {
    #[inline(always)]
    fn from(variant: BKINE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKINE`"]
pub type BKINE_R = crate::R<bool, BKINE_A>;
impl BKINE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKINE_A {
        match self.bits {
            false => BKINE_A::B_0X0,
            true => BKINE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKINE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKINE_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKINE`"]
pub struct BKINE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKINE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BKIN input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKINE_A::B_0X0)
    }
    #[doc = "BKIN input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKINE_A::B_0X1)
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
#[doc = "BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP1E_A {
    #[doc = "0: COMP1 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input enabled"]
    B_0X1 = 1,
}
impl From<BKCMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKCMP1E`"]
pub type BKCMP1E_R = crate::R<bool, BKCMP1E_A>;
impl BKCMP1E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1E_A {
        match self.bits {
            false => BKCMP1E_A::B_0X0,
            true => BKCMP1E_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKCMP1E_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKCMP1E_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKCMP1E`"]
pub struct BKCMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKCMP1E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP1 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP1E_A::B_0X0)
    }
    #[doc = "COMP1 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP1E_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP2E_A {
    #[doc = "0: COMP2 input disabled"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input enabled"]
    B_0X1 = 1,
}
impl From<BKCMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKCMP2E`"]
pub type BKCMP2E_R = crate::R<bool, BKCMP2E_A>;
impl BKCMP2E_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2E_A {
        match self.bits {
            false => BKCMP2E_A::B_0X0,
            true => BKCMP2E_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKCMP2E_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKCMP2E_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKCMP2E`"]
pub struct BKCMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2E_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKCMP2E_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP2 input disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP2E_A::B_0X0)
    }
    #[doc = "COMP2 input enabled"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP2E_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKINP_A {
    #[doc = "0: BKIN input is active low"]
    B_0X0 = 0,
    #[doc = "1: BKIN input is active high"]
    B_0X1 = 1,
}
impl From<BKINP_A> for bool {
    #[inline(always)]
    fn from(variant: BKINP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKINP`"]
pub type BKINP_R = crate::R<bool, BKINP_A>;
impl BKINP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKINP_A {
        match self.bits {
            false => BKINP_A::B_0X0,
            true => BKINP_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKINP_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKINP_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKINP`"]
pub struct BKINP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKINP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "BKIN input is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKINP_A::B_0X0)
    }
    #[doc = "BKIN input is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKINP_A::B_0X1)
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
#[doc = "BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP1P_A {
    #[doc = "0: COMP1 input is active low"]
    B_0X0 = 0,
    #[doc = "1: COMP1 input is active high"]
    B_0X1 = 1,
}
impl From<BKCMP1P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP1P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKCMP1P`"]
pub type BKCMP1P_R = crate::R<bool, BKCMP1P_A>;
impl BKCMP1P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP1P_A {
        match self.bits {
            false => BKCMP1P_A::B_0X0,
            true => BKCMP1P_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKCMP1P_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKCMP1P_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKCMP1P`"]
pub struct BKCMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKCMP1P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP1 input is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP1P_A::B_0X0)
    }
    #[doc = "COMP1 input is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP1P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BKCMP2P_A {
    #[doc = "0: COMP2 input is active low"]
    B_0X0 = 0,
    #[doc = "1: COMP2 input is active high"]
    B_0X1 = 1,
}
impl From<BKCMP2P_A> for bool {
    #[inline(always)]
    fn from(variant: BKCMP2P_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `BKCMP2P`"]
pub type BKCMP2P_R = crate::R<bool, BKCMP2P_A>;
impl BKCMP2P_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKCMP2P_A {
        match self.bits {
            false => BKCMP2P_A::B_0X0,
            true => BKCMP2P_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BKCMP2P_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BKCMP2P_A::B_0X1
    }
}
#[doc = "Write proxy for field `BKCMP2P`"]
pub struct BKCMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2P_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BKCMP2P_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "COMP2 input is active low"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(BKCMP2P_A::B_0X0)
    }
    #[doc = "COMP2 input is active high"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(BKCMP2P_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable This bit enables the BKIN alternate function input for the timerâ\u{80}\u{99}s BRK input. BKIN input is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W {
        BKINE_W { w: self }
    }
    #[doc = "Bit 1 - BRK COMP1 enable This bit enables the COMP1 for the timerâ\u{80}\u{99}s BRK input. COMP1 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W {
        BKCMP1E_W { w: self }
    }
    #[doc = "Bit 2 - BRK COMP2 enable This bit enables the COMP2 for the timerâ\u{80}\u{99}s BRK input. COMP2 output is 'ORedâ\u{80}\u{99} with the other BRK sources. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W {
        BKCMP2E_W { w: self }
    }
    #[doc = "Bit 9 - BRK BKIN input polarity This bit selects the BKIN alternate function input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W {
        BKINP_W { w: self }
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity This bit selects the COMP1 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W {
        BKCMP1P_W { w: self }
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity This bit selects the COMP2 input sensitivity. It must be programmed together with the BKP polarity bit. Note: This bit can not be modified as long as LOCK level 1 has been programmed (LOCK bits in TIMx_BDTR register)."]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W {
        BKCMP2P_W { w: self }
    }
}
