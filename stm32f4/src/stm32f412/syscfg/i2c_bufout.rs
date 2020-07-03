#[doc = "Reader of register I2C_BUFOUT"]
pub type R = crate::R<u32, super::I2C_BUFOUT>;
#[doc = "Writer for register I2C_BUFOUT"]
pub type W = crate::W<u32, super::I2C_BUFOUT>;
#[doc = "Register I2C_BUFOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_BUFOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C4SCL`"]
pub type I2C4SCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4SCL`"]
pub struct I2C4SCL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SCL_W<'a> {
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
#[doc = "Reader of field `I2C4SDA`"]
pub type I2C4SDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4SDA`"]
pub struct I2C4SDA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SDA_W<'a> {
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
impl R {
    #[doc = "Bit 0 - I2C4SCL"]
    #[inline(always)]
    pub fn i2c4scl(&self) -> I2C4SCL_R {
        I2C4SCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4SDA"]
    #[inline(always)]
    pub fn i2c4sda(&self) -> I2C4SDA_R {
        I2C4SDA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C4SCL"]
    #[inline(always)]
    pub fn i2c4scl(&mut self) -> I2C4SCL_W {
        I2C4SCL_W { w: self }
    }
    #[doc = "Bit 1 - I2C4SDA"]
    #[inline(always)]
    pub fn i2c4sda(&mut self) -> I2C4SDA_W {
        I2C4SDA_W { w: self }
    }
}
