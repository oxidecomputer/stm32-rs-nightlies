#[doc = "Reader of register RTSR2"]
pub type R = crate::R<u32, super::RTSR2>;
#[doc = "Writer for register RTSR2"]
pub type W = crate::W<u32, super::RTSR2>;
#[doc = "Register RTSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TR49`"]
pub type TR49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR49`"]
pub struct TR49_W<'a> {
    w: &'a mut W,
}
impl<'a> TR49_W<'a> {
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
#[doc = "Reader of field `TR51`"]
pub type TR51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TR51`"]
pub struct TR51_W<'a> {
    w: &'a mut W,
}
impl<'a> TR51_W<'a> {
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
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&self) -> TR49_R {
        TR49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&self) -> TR51_R {
        TR51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&mut self) -> TR49_W {
        TR49_W { w: self }
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&mut self) -> TR51_W {
        TR51_W { w: self }
    }
}
