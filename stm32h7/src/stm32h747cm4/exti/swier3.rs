#[doc = "Reader of register SWIER3"]
pub type R = crate::R<u32, super::SWIER3>;
#[doc = "Writer for register SWIER3"]
pub type W = crate::W<u32, super::SWIER3>;
#[doc = "Register SWIER3 `reset()`'s with value 0"]
impl crate::ResetValue for super::SWIER3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWIER82`"]
pub type SWIER82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIER82`"]
pub struct SWIER82_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER82_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SWIER84`"]
pub type SWIER84_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIER84`"]
pub struct SWIER84_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER84_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `SWIER85`"]
pub type SWIER85_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIER85`"]
pub struct SWIER85_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER85_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SWIER86`"]
pub type SWIER86_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWIER86`"]
pub struct SWIER86_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIER86_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> SWIER82_R {
        SWIER82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> SWIER84_R {
        SWIER84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> SWIER85_R {
        SWIER85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> SWIER86_R {
        SWIER86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&mut self) -> SWIER82_W {
        SWIER82_W { w: self }
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&mut self) -> SWIER84_W {
        SWIER84_W { w: self }
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&mut self) -> SWIER85_W {
        SWIER85_W { w: self }
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&mut self) -> SWIER86_W {
        SWIER86_W { w: self }
    }
}
