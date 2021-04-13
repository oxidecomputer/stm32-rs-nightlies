#[doc = "Reader of register SCR"]
pub type R = crate::R<u32, super::SCR>;
#[doc = "Writer for register SCR"]
pub type W = crate::W<u32, super::SCR>;
#[doc = "Register SCR `reset()`'s with value 0"]
impl crate::ResetValue for super::SCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CALRAF`"]
pub type CALRAF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALRAF`"]
pub struct CALRAF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRAF_W<'a> {
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
#[doc = "Reader of field `CALRBF`"]
pub type CALRBF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CALRBF`"]
pub struct CALRBF_W<'a> {
    w: &'a mut W,
}
impl<'a> CALRBF_W<'a> {
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
#[doc = "Reader of field `CWUTF`"]
pub type CWUTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CWUTF`"]
pub struct CWUTF_W<'a> {
    w: &'a mut W,
}
impl<'a> CWUTF_W<'a> {
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
#[doc = "Reader of field `CTSF`"]
pub type CTSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSF`"]
pub struct CTSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSF_W<'a> {
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
#[doc = "Reader of field `CTSOVF`"]
pub type CTSOVF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTSOVF`"]
pub struct CTSOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSOVF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CITSF`"]
pub type CITSF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CITSF`"]
pub struct CITSF_W<'a> {
    w: &'a mut W,
}
impl<'a> CITSF_W<'a> {
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
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    pub fn calraf(&self) -> CALRAF_R {
        CALRAF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    pub fn calrbf(&self) -> CALRBF_R {
        CALRBF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    pub fn cwutf(&self) -> CWUTF_R {
        CWUTF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&self) -> CTSF_R {
        CTSF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    pub fn ctsovf(&self) -> CTSOVF_R {
        CTSOVF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    pub fn citsf(&self) -> CITSF_R {
        CITSF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CALRAF"]
    #[inline(always)]
    pub fn calraf(&mut self) -> CALRAF_W {
        CALRAF_W { w: self }
    }
    #[doc = "Bit 1 - CALRBF"]
    #[inline(always)]
    pub fn calrbf(&mut self) -> CALRBF_W {
        CALRBF_W { w: self }
    }
    #[doc = "Bit 2 - CWUTF"]
    #[inline(always)]
    pub fn cwutf(&mut self) -> CWUTF_W {
        CWUTF_W { w: self }
    }
    #[doc = "Bit 3 - CTSF"]
    #[inline(always)]
    pub fn ctsf(&mut self) -> CTSF_W {
        CTSF_W { w: self }
    }
    #[doc = "Bit 4 - CTSOVF"]
    #[inline(always)]
    pub fn ctsovf(&mut self) -> CTSOVF_W {
        CTSOVF_W { w: self }
    }
    #[doc = "Bit 5 - CITSF"]
    #[inline(always)]
    pub fn citsf(&mut self) -> CITSF_W {
        CITSF_W { w: self }
    }
}
