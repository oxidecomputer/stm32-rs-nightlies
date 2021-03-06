#[doc = "Reader of register MACWTR"]
pub type R = crate::R<u32, super::MACWTR>;
#[doc = "Writer for register MACWTR"]
pub type W = crate::W<u32, super::MACWTR>;
#[doc = "Register MACWTR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACWTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTO`"]
pub type WTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WTO`"]
pub struct WTO_W<'a> {
    w: &'a mut W,
}
impl<'a> WTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `PWE`"]
pub type PWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWE`"]
pub struct PWE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&self) -> WTO_R {
        WTO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&self) -> PWE_R {
        PWE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout"]
    #[inline(always)]
    pub fn wto(&mut self) -> WTO_W {
        WTO_W { w: self }
    }
    #[doc = "Bit 8 - Programmable Watchdog Enable"]
    #[inline(always)]
    pub fn pwe(&mut self) -> PWE_W {
        PWE_W { w: self }
    }
}
