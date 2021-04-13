#[doc = "Reader of register CRC_CR"]
pub type R = crate::R<u32, super::CRC_CR>;
#[doc = "Writer for register CRC_CR"]
pub type W = crate::W<u32, super::CRC_CR>;
#[doc = "Register CRC_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reverse output data This bit controls the reversal of the bit order of the output data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REV_OUT_A {
    #[doc = "0: Bit order not affected"]
    B_0X0 = 0,
    #[doc = "1: Bit-reversed output format"]
    B_0X1 = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REV_OUT`"]
pub type REV_OUT_R = crate::R<bool, REV_OUT_A>;
impl REV_OUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::B_0X0,
            true => REV_OUT_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_OUT_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_OUT_A::B_0X1
    }
}
#[doc = "Write proxy for field `REV_OUT`"]
pub struct REV_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_OUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV_OUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REV_OUT_A::B_0X0)
    }
    #[doc = "Bit-reversed output format"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REV_OUT_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reverse input data These bits control the reversal of the bit order of the input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REV_IN_A {
    #[doc = "0: Bit order not affected"]
    B_0X0 = 0,
    #[doc = "1: Bit reversal done by byte"]
    B_0X1 = 1,
    #[doc = "2: Bit reversal done by half-word"]
    B_0X2 = 2,
    #[doc = "3: Bit reversal done by word"]
    B_0X3 = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `REV_IN`"]
pub type REV_IN_R = crate::R<u8, REV_IN_A>;
impl REV_IN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::B_0X0,
            1 => REV_IN_A::B_0X1,
            2 => REV_IN_A::B_0X2,
            3 => REV_IN_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REV_IN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REV_IN_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == REV_IN_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == REV_IN_A::B_0X3
    }
}
#[doc = "Write proxy for field `REV_IN`"]
pub struct REV_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_IN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REV_IN_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X0)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X1)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X2)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(REV_IN_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | (((value as u32) & 0x03) << 5);
        self.w
    }
}
#[doc = "Polynomial size These bits control the size of the polynomial.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    #[doc = "0: 32 bit polynomial"]
    B_0X0 = 0,
    #[doc = "1: 16 bit polynomial"]
    B_0X1 = 1,
    #[doc = "2: 8 bit polynomial"]
    B_0X2 = 2,
    #[doc = "3: 7 bit polynomial"]
    B_0X3 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `POLYSIZE`"]
pub type POLYSIZE_R = crate::R<u8, POLYSIZE_A>;
impl POLYSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::B_0X0,
            1 => POLYSIZE_A::B_0X1,
            2 => POLYSIZE_A::B_0X2,
            3 => POLYSIZE_A::B_0X3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == POLYSIZE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == POLYSIZE_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == POLYSIZE_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == POLYSIZE_A::B_0X3
    }
}
#[doc = "Write proxy for field `POLYSIZE`"]
pub struct POLYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> POLYSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLYSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "32 bit polynomial"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X0)
    }
    #[doc = "16 bit polynomial"]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X1)
    }
    #[doc = "8 bit polynomial"]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X2)
    }
    #[doc = "7 bit polynomial"]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(POLYSIZE_A::B_0X3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Reverse output data This bit controls the reversal of the bit order of the output data."]
    #[inline(always)]
    pub fn rev_out(&mut self) -> REV_OUT_W {
        REV_OUT_W { w: self }
    }
    #[doc = "Bits 5:6 - Reverse input data These bits control the reversal of the bit order of the input data"]
    #[inline(always)]
    pub fn rev_in(&mut self) -> REV_IN_W {
        REV_IN_W { w: self }
    }
    #[doc = "Bits 3:4 - Polynomial size These bits control the size of the polynomial."]
    #[inline(always)]
    pub fn polysize(&mut self) -> POLYSIZE_W {
        POLYSIZE_W { w: self }
    }
    #[doc = "Bit 0 - RESET bit"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
}
