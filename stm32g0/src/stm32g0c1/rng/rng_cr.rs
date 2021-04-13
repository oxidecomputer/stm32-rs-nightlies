#[doc = "Reader of register RNG_CR"]
pub type R = crate::R<u32, super::RNG_CR>;
#[doc = "Writer for register RNG_CR"]
pub type W = crate::W<u32, super::RNG_CR>;
#[doc = "Register RNG_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::RNG_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "True random number generator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RNGEN_A {
    #[doc = "0: True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    B_0X0 = 0,
    #[doc = "1: True random number generator is enabled."]
    B_0X1 = 1,
}
impl From<RNGEN_A> for bool {
    #[inline(always)]
    fn from(variant: RNGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RNGEN`"]
pub type RNGEN_R = crate::R<bool, RNGEN_A>;
impl RNGEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGEN_A {
        match self.bits {
            false => RNGEN_A::B_0X0,
            true => RNGEN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RNGEN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RNGEN_A::B_0X1
    }
}
#[doc = "Write proxy for field `RNGEN`"]
pub struct RNGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "True random number generator is disabled. Analog noise sources are powered off and logic clocked by the RNG clock is gated."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(RNGEN_A::B_0X0)
    }
    #[doc = "True random number generator is enabled."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(RNGEN_A::B_0X1)
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
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: RNG Interrupt is disabled"]
    B_0X0 = 0,
    #[doc = "1: RNG Interrupt is enabled. An interrupt is pending as soon as DRDY='1', SEIS='1' or CEIS=1 in the RNG_SR register."]
    B_0X1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IE`"]
pub type IE_R = crate::R<bool, IE_A>;
impl IE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::B_0X0,
            true => IE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == IE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == IE_A::B_0X1
    }
}
#[doc = "Write proxy for field `IE`"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RNG Interrupt is disabled"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(IE_A::B_0X0)
    }
    #[doc = "RNG Interrupt is enabled. An interrupt is pending as soon as DRDY='1', SEIS='1' or CEIS=1 in the RNG_SR register."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(IE_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CED_A {
    #[doc = "0: Clock error detection is enable"]
    B_0X0 = 0,
    #[doc = "1: Clock error detection is disable"]
    B_0X1 = 1,
}
impl From<CED_A> for bool {
    #[inline(always)]
    fn from(variant: CED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CED`"]
pub type CED_R = crate::R<bool, CED_A>;
impl CED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CED_A {
        match self.bits {
            false => CED_A::B_0X0,
            true => CED_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CED_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CED_A::B_0X1
    }
}
#[doc = "Write proxy for field `CED`"]
pub struct CED_W<'a> {
    w: &'a mut W,
}
impl<'a> CED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock error detection is enable"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CED_A::B_0X0)
    }
    #[doc = "Clock error detection is disable"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CED_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    pub fn ced(&self) -> CED_R {
        CED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - True random number generator enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W {
        RNGEN_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 5 - Clock error detection The clock error detection cannot be enabled nor disabled on-the-fly when the RNG is enabled, i.e. to enable or disable CED the RNG must be disabled."]
    #[inline(always)]
    pub fn ced(&mut self) -> CED_W {
        CED_W { w: self }
    }
}
