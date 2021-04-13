#[doc = "Reader of register FDCAN_TOCC"]
pub type R = crate::R<u32, super::FDCAN_TOCC>;
#[doc = "Writer for register FDCAN_TOCC"]
pub type W = crate::W<u32, super::FDCAN_TOCC>;
#[doc = "Register FDCAN_TOCC `reset()`'s with value 0xffff_0000"]
impl crate::ResetValue for super::FDCAN_TOCC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_0000
    }
}
#[doc = "Reader of field `ETOC`"]
pub type ETOC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ETOC`"]
pub struct ETOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETOC_W<'a> {
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
#[doc = "Reader of field `TOS`"]
pub type TOS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TOS`"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `TOP`"]
pub type TOP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TOP`"]
pub struct TOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&self) -> ETOC_R {
        ETOC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - ETOC"]
    #[inline(always)]
    pub fn etoc(&mut self) -> ETOC_W {
        ETOC_W { w: self }
    }
    #[doc = "Bits 1:2 - TOS"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:31 - TOP"]
    #[inline(always)]
    pub fn top(&mut self) -> TOP_W {
        TOP_W { w: self }
    }
}