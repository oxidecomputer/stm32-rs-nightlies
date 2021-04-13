#[doc = "Reader of register I2C_OAR1"]
pub type R = crate::R<u32, super::I2C_OAR1>;
#[doc = "Writer for register I2C_OAR1"]
pub type W = crate::W<u32, super::I2C_OAR1>;
#[doc = "Register I2C_OAR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_OAR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OA1`"]
pub type OA1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `OA1`"]
pub struct OA1_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1MODE_A {
    #[doc = "0: Own address 1 is a 7-bit address."]
    B_0X0 = 0,
    #[doc = "1: Own address 1 is a 10-bit address."]
    B_0X1 = 1,
}
impl From<OA1MODE_A> for bool {
    #[inline(always)]
    fn from(variant: OA1MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OA1MODE`"]
pub type OA1MODE_R = crate::R<bool, OA1MODE_A>;
impl OA1MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1MODE_A {
        match self.bits {
            false => OA1MODE_A::B_0X0,
            true => OA1MODE_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA1MODE_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA1MODE_A::B_0X1
    }
}
#[doc = "Write proxy for field `OA1MODE`"]
pub struct OA1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA1MODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Own address 1 is a 7-bit address."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA1MODE_A::B_0X0)
    }
    #[doc = "Own address 1 is a 10-bit address."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA1MODE_A::B_0X1)
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
#[doc = "Own Address 1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OA1EN_A {
    #[doc = "0: Own address 1 disabled. The received slave address OA1 is NACKed."]
    B_0X0 = 0,
    #[doc = "1: Own address 1 enabled. The received slave address OA1 is ACKed."]
    B_0X1 = 1,
}
impl From<OA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: OA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OA1EN`"]
pub type OA1EN_R = crate::R<bool, OA1EN_A>;
impl OA1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OA1EN_A {
        match self.bits {
            false => OA1EN_A::B_0X0,
            true => OA1EN_A::B_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `B_0X0`"]
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OA1EN_A::B_0X0
    }
    #[doc = "Checks if the value of the field is `B_0X1`"]
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OA1EN_A::B_0X1
    }
}
#[doc = "Write proxy for field `OA1EN`"]
pub struct OA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OA1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Own address 1 disabled. The received slave address OA1 is NACKed."]
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut W {
        self.variant(OA1EN_A::B_0X0)
    }
    #[doc = "Own address 1 enabled. The received slave address OA1 is ACKed."]
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut W {
        self.variant(OA1EN_A::B_0X1)
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
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Interface own slave address 7-bit addressing mode: OA1\\[7:1\\]
contains the 7-bit own slave address. The bits OA1\\[9\\], OA1\\[8\\]
and OA1\\[0\\]
are don't care. 10-bit addressing mode: OA1\\[9:0\\]
contains the 10-bit own slave address. Note: These bits can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1(&mut self) -> OA1_W {
        OA1_W { w: self }
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode Note: This bit can be written only when OA1EN=0."]
    #[inline(always)]
    pub fn oa1mode(&mut self) -> OA1MODE_W {
        OA1MODE_W { w: self }
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn oa1en(&mut self) -> OA1EN_W {
        OA1EN_W { w: self }
    }
}
