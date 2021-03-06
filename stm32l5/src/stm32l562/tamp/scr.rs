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
#[doc = "Write proxy for field `CTAMP1F`"]
pub struct CTAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP1F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP2F`"]
pub struct CTAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP2F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP3F`"]
pub struct CTAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP3F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP4F`"]
pub struct CTAMP4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP4F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP5F`"]
pub struct CTAMP5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP5F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP6F`"]
pub struct CTAMP6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP6F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP7F`"]
pub struct CTAMP7F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP7F_W<'a> {
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
#[doc = "Write proxy for field `CTAMP8F`"]
pub struct CTAMP8F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP8F_W<'a> {
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
#[doc = "Write proxy for field `CITAMP1F`"]
pub struct CITAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP1F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `CITAMP2F`"]
pub struct CITAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP2F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `CITAMP3F`"]
pub struct CITAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP3F_W<'a> {
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
#[doc = "Write proxy for field `CITAMP5F`"]
pub struct CITAMP5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP5F_W<'a> {
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
#[doc = "Write proxy for field `CITAMP8F`"]
pub struct CITAMP8F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP8F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W {
        CTAMP1F_W { w: self }
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W {
        CTAMP2F_W { w: self }
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W {
        CTAMP3F_W { w: self }
    }
    #[doc = "Bit 3 - CTAMP4F"]
    #[inline(always)]
    pub fn ctamp4f(&mut self) -> CTAMP4F_W {
        CTAMP4F_W { w: self }
    }
    #[doc = "Bit 4 - CTAMP5F"]
    #[inline(always)]
    pub fn ctamp5f(&mut self) -> CTAMP5F_W {
        CTAMP5F_W { w: self }
    }
    #[doc = "Bit 5 - CTAMP6F"]
    #[inline(always)]
    pub fn ctamp6f(&mut self) -> CTAMP6F_W {
        CTAMP6F_W { w: self }
    }
    #[doc = "Bit 6 - CTAMP7F"]
    #[inline(always)]
    pub fn ctamp7f(&mut self) -> CTAMP7F_W {
        CTAMP7F_W { w: self }
    }
    #[doc = "Bit 7 - CTAMP8F"]
    #[inline(always)]
    pub fn ctamp8f(&mut self) -> CTAMP8F_W {
        CTAMP8F_W { w: self }
    }
    #[doc = "Bit 16 - CITAMP1F"]
    #[inline(always)]
    pub fn citamp1f(&mut self) -> CITAMP1F_W {
        CITAMP1F_W { w: self }
    }
    #[doc = "Bit 17 - CITAMP2F"]
    #[inline(always)]
    pub fn citamp2f(&mut self) -> CITAMP2F_W {
        CITAMP2F_W { w: self }
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W {
        CITAMP3F_W { w: self }
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W {
        CITAMP5F_W { w: self }
    }
    #[doc = "Bit 23 - CITAMP8F"]
    #[inline(always)]
    pub fn citamp8f(&mut self) -> CITAMP8F_W {
        CITAMP8F_W { w: self }
    }
}
