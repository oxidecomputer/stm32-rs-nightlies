#[doc = "Reader of register I2C_OAR2"]
pub type R = crate::R<u32, super::I2C_OAR2>;
#[doc = "Writer for register I2C_OAR2"]
pub type W = crate::W<u32, super::I2C_OAR2>;
#[doc = "Register I2C_OAR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_OAR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA2`"]
pub type OA2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OA2`"]
pub struct OA2_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | (((value as u32) & 0x7f) << 1);
        self.w
    }
}
#[doc = "Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OA2MSK_A {
    #[doc = "0: No mask"]
    B_0X0 = 0,
    #[doc = "1: OA2\\[1\\]
is masked and donâ\u{80}\u{99}t care. Only OA2\\[7:2\\]
are compared."]
    B_0X1 = 1,
    #[doc = "2: OA2\\[2:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:3\\]
are compared."]
    B_0X2 = 2,
    #[doc = "3: OA2\\[3:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:4\\]
are compared."]
    B_0X3 = 3,
    #[doc = "4: OA2\\[4:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:5\\]
are compared."]
    B_0X4 = 4,
    #[doc = "5: OA2\\[5:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:6\\]
are compared."]
    B_0X5 = 5,
    #[doc = "6: OA2\\[6:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7\\]
is compared."]
    B_0X6 = 6,
    #[doc = "7: OA2\\[7:1\\]
are masked and donâ\u{80}\u{99}t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    B_0X7 = 7,
}
impl From<OA2MSK_A> for u8 {
    #[inline(always)]
    fn from(variant: OA2MSK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OA2MSK`"]
pub type OA2MSK_R = crate::R<u8, OA2MSK_A>;
impl OA2MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2MSK_A {
        match self.bits {
            0 => OA2MSK_A::B_0X0,
            1 => OA2MSK_A::B_0X1,
            2 => OA2MSK_A::B_0X2,
            3 => OA2MSK_A::B_0X3,
            4 => OA2MSK_A::B_0X4,
            5 => OA2MSK_A::B_0X5,
            6 => OA2MSK_A::B_0X6,
            7 => OA2MSK_A::B_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2MSK_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2MSK_A::B_0X1
    }
    #[doc = "Checks if the value of the field is `B_0X2`"]
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == OA2MSK_A::B_0X2
    }
    #[doc = "Checks if the value of the field is `B_0X3`"]
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == OA2MSK_A::B_0X3
    }
    #[doc = "Checks if the value of the field is `B_0X4`"]
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == OA2MSK_A::B_0X4
    }
    #[doc = "Checks if the value of the field is `B_0X5`"]
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == OA2MSK_A::B_0X5
    }
    #[doc = "Checks if the value of the field is `B_0X6`"]
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == OA2MSK_A::B_0X6
    }
    #[doc = "Checks if the value of the field is `B_0X7`"]
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == OA2MSK_A::B_0X7
    }
}
#[doc = "Write proxy for field `OA2MSK`"]
pub struct OA2MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA2MSK_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No mask"]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X0)
    }
    #[doc = "OA2\\[1\\]
is masked and donâ\u{80}\u{99}t care. Only OA2\\[7:2\\]
are compared."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X1)
    }
    #[doc = "OA2\\[2:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:3\\]
are compared."]
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X2)
    }
    #[doc = "OA2\\[3:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:4\\]
are compared."]
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X3)
    }
    #[doc = "OA2\\[4:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:5\\]
are compared."]
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X4)
    }
    #[doc = "OA2\\[5:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7:6\\]
are compared."]
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X5)
    }
    #[doc = "OA2\\[6:1\\]
are masked and donâ\u{80}\u{99}t care. Only OA2\\[7\\]
is compared."]
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X6)
    }
    #[doc = "OA2\\[7:1\\]
are masked and donâ\u{80}\u{99}t care. No comparison is done, and all (except reserved) 7-bit received addresses are acknowledged."]
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut W {
        self.variant(OA2MSK_A::B_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Own Address 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA2EN_A {
    #[doc = "0: Own address 2 disabled. The received slave address OA2 is NACKed."]
    B_0X0 = 0,
    #[doc = "1: Own address 2 enabled. The received slave address OA2 is ACKed."]
    B_0X1 = 1,
}
impl From<OA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OA2EN`"]
pub type OA2EN_R = crate::R<bool, OA2EN_A>;
impl OA2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA2EN_A {
        match self.bits {
            false => OA2EN_A::B_0X0,
            true => OA2EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA2EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA2EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `OA2EN`"]
pub struct OA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Own address 2 disabled. The received slave address OA2 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA2EN_A::B_0X0)
    }
    #[doc = "Own address 2 enabled. The received slave address OA2 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA2EN_A::B_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&self) -> OA2_R {
        OA2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&self) -> OA2MSK_R {
        OA2MSK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&self) -> OA2EN_R {
        OA2EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:7 - Interface address 7-bit addressing mode: 7-bit address Note: These bits can be written only when OA2EN=0."]
    #[inline(always)]
    pub fn oa2(&mut self) -> OA2_W {
        OA2_W { w: self }
    }
    #[doc = "Bits 8:10 - Own Address 2 masks Note: These bits can be written only when OA2EN=0. As soon as OA2MSK is not equal to 0, the reserved I2C addresses (0b0000xxx and 0b1111xxx) are not acknowledged even if the comparison matches."]
    #[inline(always)]
    pub fn oa2msk(&mut self) -> OA2MSK_W {
        OA2MSK_W { w: self }
    }
    #[doc = "Bit 15 - Own Address 2 enable"]
    #[inline(always)]
    pub fn oa2en(&mut self) -> OA2EN_W {
        OA2EN_W { w: self }
    }
}
