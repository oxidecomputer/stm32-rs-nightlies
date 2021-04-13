#[doc = "Reader of register SPI_CRCPOLY"]
pub type R = crate::R<u32, super::SPI_CRCPOLY>;
#[doc = "Writer for register SPI_CRCPOLY"]
pub type W = crate::W<u32, super::SPI_CRCPOLY>;
#[doc = "Register SPI_CRCPOLY `reset()`'s with value 0x0107"]
impl crate::ResetValue for super::SPI_CRCPOLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0107
    }
}
#[doc = "Reader of field `CRCPOLY`"]
pub type CRCPOLY_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CRCPOLY`"]
pub struct CRCPOLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCPOLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRCPOLY"]
    #[inline(always)]
    pub fn crcpoly(&mut self) -> CRCPOLY_W {
        CRCPOLY_W { w: self }
    }
}
