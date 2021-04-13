#[doc = "Reader of register DSI_LCOLCR"]
pub type R = crate::R<u32, super::DSI_LCOLCR>;
#[doc = "Writer for register DSI_LCOLCR"]
pub type W = crate::W<u32, super::DSI_LCOLCR>;
#[doc = "Register DSI_LCOLCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_LCOLCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COLC`"]
pub type COLC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `COLC`"]
pub struct COLC_W<'a> {
    w: &'a mut W,
}
impl<'a> COLC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `LPE`"]
pub type LPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPE`"]
pub struct LPE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPE_W<'a> {
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
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> COLC_R {
        COLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LPE_R {
        LPE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&mut self) -> COLC_W {
        COLC_W { w: self }
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&mut self) -> LPE_W {
        LPE_W { w: self }
    }
}
