#[doc = "Reader of register ETH_MACA2HR"]
pub type R = crate::R<u32, super::ETH_MACA2HR>;
#[doc = "Writer for register ETH_MACA2HR"]
pub type W = crate::W<u32, super::ETH_MACA2HR>;
#[doc = "Register ETH_MACA2HR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ETH_MACA2HR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `ADDRHI`"]
pub type ADDRHI_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADDRHI`"]
pub struct ADDRHI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRHI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `MBC`"]
pub type MBC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MBC`"]
pub struct MBC_W<'a> {
    w: &'a mut W,
}
impl<'a> MBC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
#[doc = "Reader of field `SA`"]
pub type SA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SA`"]
pub struct SA_W<'a> {
    w: &'a mut W,
}
impl<'a> SA_W<'a> {
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
#[doc = "Reader of field `AE`"]
pub type AE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AE`"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
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
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&self) -> ADDRHI_R {
        ADDRHI_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDRHI"]
    #[inline(always)]
    pub fn addrhi(&mut self) -> ADDRHI_W {
        ADDRHI_W { w: self }
    }
    #[doc = "Bits 24:29 - MBC"]
    #[inline(always)]
    pub fn mbc(&mut self) -> MBC_W {
        MBC_W { w: self }
    }
    #[doc = "Bit 30 - SA"]
    #[inline(always)]
    pub fn sa(&mut self) -> SA_W {
        SA_W { w: self }
    }
    #[doc = "Bit 31 - AE"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
}
