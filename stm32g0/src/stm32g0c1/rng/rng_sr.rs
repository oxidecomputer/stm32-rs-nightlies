#[doc = "Reader of register RNG_SR"]
pub type R = crate::R<u32, super::RNG_SR>;
#[doc = "Writer for register RNG_SR"]
pub type W = crate::W<u32, super::RNG_SR>;
#[doc = "Register RNG_SR `reset()`'s with value 0"]
impl crate::ResetValue for super::RNG_SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRDY_A {
    #[doc = "0: The RNG_DR register is not yet valid, no random data is available."]
    B_0X0 = 0,
    #[doc = "1: The RNG_DR register contains valid random data."]
    B_0X1 = 1,
}
impl From<DRDY_A> for bool {
    #[inline(always)]
    fn from(variant: DRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DRDY`"]
pub type DRDY_R = crate::R<bool, DRDY_A>;
impl DRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRDY_A {
        match self.bits {
            false => DRDY_A::B_0X0,
            true => DRDY_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DRDY_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DRDY_A::B_0X1
    }
}
#[doc = "Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32). If the CEIS bit is set, this means that a slow clock was detected and the situation has been recovered."]
    B_0X0 = 0,
    #[doc = "1: The RNG clock is too slow (fRNGCLK< fHCLK/32)."]
    B_0X1 = 1,
}
impl From<CECS_A> for bool {
    #[inline(always)]
    fn from(variant: CECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CECS`"]
pub type CECS_R = crate::R<bool, CECS_A>;
impl CECS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECS_A {
        match self.bits {
            false => CECS_A::B_0X0,
            true => CECS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CECS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CECS_A::B_0X1
    }
}
#[doc = "Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECS_A {
    #[doc = "0: No faulty sequence has currently been detected. If the SEIS bit is set, this means that a faulty sequence was detected and the situation has been recovered."]
    B_0X0 = 0,
    #[doc = "1: At least one of the following faulty sequence has been detected:"]
    B_0X1 = 1,
}
impl From<SECS_A> for bool {
    #[inline(always)]
    fn from(variant: SECS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SECS`"]
pub type SECS_R = crate::R<bool, SECS_A>;
impl SECS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECS_A {
        match self.bits {
            false => SECS_A::B_0X0,
            true => SECS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SECS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SECS_A::B_0X1
    }
}
#[doc = "Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEIS_A {
    #[doc = "0: The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    B_0X0 = 0,
    #[doc = "1: The RNG has been detected too slow (fRNGCLK< fHCLK/32)"]
    B_0X1 = 1,
}
impl From<CEIS_A> for bool {
    #[inline(always)]
    fn from(variant: CEIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CEIS`"]
pub type CEIS_R = crate::R<bool, CEIS_A>;
impl CEIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEIS_A {
        match self.bits {
            false => CEIS_A::B_0X0,
            true => CEIS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CEIS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CEIS_A::B_0X1
    }
}
#[doc = "Write proxy for field `CEIS`"]
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The RNG clock is correct (fRNGCLK> fHCLK/32)"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(CEIS_A::B_0X0)
    }
    #[doc = "The RNG has been detected too slow (fRNGCLK< fHCLK/32)"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(CEIS_A::B_0X1)
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
#[doc = "Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEIS_A {
    #[doc = "0: No faulty sequence detected"]
    B_0X0 = 0,
    #[doc = "1: At least one faulty sequence has been detected. See SECS bit description for details."]
    B_0X1 = 1,
}
impl From<SEIS_A> for bool {
    #[inline(always)]
    fn from(variant: SEIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SEIS`"]
pub type SEIS_R = crate::R<bool, SEIS_A>;
impl SEIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIS_A {
        match self.bits {
            false => SEIS_A::B_0X0,
            true => SEIS_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SEIS_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SEIS_A::B_0X1
    }
}
#[doc = "Write proxy for field `SEIS`"]
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No faulty sequence detected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(SEIS_A::B_0X0)
    }
    #[doc = "At least one faulty sequence has been detected. See SECS bit description for details."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(SEIS_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready Once the output buffer becomes empty (after reading the RNG_DR register), this bit returns to 0 until a new random value is generated. Note: The DRDY bit can rise when the peripheral is disabled (RNGEN=0 in the RNG_CR register). If IE=1 in the RNG_CR register, an interrupt is generated when DRDY=1."]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock error current status Note: CECS bit is valid only if the CED bit in the RNG_CR register is set to 0."]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Seed error current status One of the noise source has provided more than 64 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 32 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d}) Both noise sources have delivered more than 32 consecutive bits at a constant value (â\u{80}\u{9c}0â\u{80}\u{9d} or â\u{80}\u{9c}1â\u{80}\u{9d}), or more than 16 consecutive occurrence of two bit patterns (â\u{80}\u{9c}01â\u{80}\u{9d} or â\u{80}\u{9c}10â\u{80}\u{9d})"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock error interrupt status This bit is set at the same time as CECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
    #[doc = "Bit 6 - Seed error interrupt status This bit is set at the same time as SECS. It is cleared by writing 0. Writing 1 has no effect. An interrupt is pending if IE = 1 in the RNG_CR register."]
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
}
