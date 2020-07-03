#[doc = "Reader of register C2PR2"]
pub type R = crate::R<u32, super::C2PR2>;
#[doc = "Writer for register C2PR2"]
pub type W = crate::W<u32, super::C2PR2>;
#[doc = "Register C2PR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2PR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PR49`"]
pub type PR49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR49`"]
pub struct PR49_W<'a> {
    w: &'a mut W,
}
impl<'a> PR49_W<'a> {
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
#[doc = "Reader of field `PR51`"]
pub type PR51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PR51`"]
pub struct PR51_W<'a> {
    w: &'a mut W,
}
impl<'a> PR51_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 17 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> PR49_R {
        PR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> PR51_R {
        PR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&mut self) -> PR49_W {
        PR49_W { w: self }
    }
    #[doc = "Bit 19 - CPU2 configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&mut self) -> PR51_W {
        PR51_W { w: self }
    }
}
