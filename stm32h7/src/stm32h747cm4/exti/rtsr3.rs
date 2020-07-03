#[doc = "Reader of register RTSR3"]
pub type R = crate::R<u32, super::RTSR3>;
#[doc = "Writer for register RTSR3"]
pub type W = crate::W<u32, super::RTSR3>;
#[doc = "Register RTSR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTSR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR82`"]
pub type TR82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR82`"]
pub struct TR82_W<'a> {
    w: &'a mut W,
}
impl<'a> TR82_W<'a> {
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
#[doc = "Reader of field `TR84`"]
pub type TR84_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR84`"]
pub struct TR84_W<'a> {
    w: &'a mut W,
}
impl<'a> TR84_W<'a> {
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
#[doc = "Reader of field `TR85`"]
pub type TR85_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR85`"]
pub struct TR85_W<'a> {
    w: &'a mut W,
}
impl<'a> TR85_W<'a> {
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
#[doc = "Reader of field `TR86`"]
pub type TR86_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR86`"]
pub struct TR86_W<'a> {
    w: &'a mut W,
}
impl<'a> TR86_W<'a> {
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
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> TR82_R {
        TR82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> TR84_R {
        TR84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> TR85_R {
        TR85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> TR86_R {
        TR86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&mut self) -> TR82_W {
        TR82_W { w: self }
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&mut self) -> TR84_W {
        TR84_W { w: self }
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&mut self) -> TR85_W {
        TR85_W { w: self }
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&mut self) -> TR86_W {
        TR86_W { w: self }
    }
}
