#[doc = "Reader of register FDCAN_TTRMC"]
pub type R = crate::R<u32, super::FDCAN_TTRMC>;
#[doc = "Writer for register FDCAN_TTRMC"]
pub type W = crate::W<u32, super::FDCAN_TTRMC>;
#[doc = "Register FDCAN_TTRMC `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTRMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RID`"]
pub type RID_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RID`"]
pub struct RID_W<'a> {
    w: &'a mut W,
}
impl<'a> RID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Reader of field `XTD`"]
pub type XTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `XTD`"]
pub struct XTD_W<'a> {
    w: &'a mut W,
}
impl<'a> XTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RMPS`"]
pub type RMPS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMPS`"]
pub struct RMPS_W<'a> {
    w: &'a mut W,
}
impl<'a> RMPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - RID"]
    #[inline(always)]
    pub fn rid(&self) -> RID_R {
        RID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 30 - XTD"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - RMPS"]
    #[inline(always)]
    pub fn rmps(&self) -> RMPS_R {
        RMPS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - RID"]
    #[inline(always)]
    pub fn rid(&mut self) -> RID_W {
        RID_W { w: self }
    }
    #[doc = "Bit 30 - XTD"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W { w: self }
    }
    #[doc = "Bit 31 - RMPS"]
    #[inline(always)]
    pub fn rmps(&mut self) -> RMPS_W {
        RMPS_W { w: self }
    }
}
