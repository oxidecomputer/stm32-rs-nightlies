#[doc = "Reader of register SPI_UDRDR"]
pub type R = crate::R<u32, super::SPI_UDRDR>;
#[doc = "Writer for register SPI_UDRDR"]
pub type W = crate::W<u32, super::SPI_UDRDR>;
#[doc = "Register SPI_UDRDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_UDRDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UDRDR`"]
pub type UDRDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UDRDR`"]
pub struct UDRDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - UDRDR"]
    #[inline(always)]
    pub fn udrdr(&mut self) -> UDRDR_W {
        UDRDR_W { w: self }
    }
}
