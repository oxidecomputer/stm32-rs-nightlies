#[doc = "Reader of register BDMA_CPAR5"]
pub type R = crate::R<u32, super::BDMA_CPAR5>;
#[doc = "Writer for register BDMA_CPAR5"]
pub type W = crate::W<u32, super::BDMA_CPAR5>;
#[doc = "Register BDMA_CPAR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::BDMA_CPAR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PA`"]
pub type PA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PA`"]
pub struct PA_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\]
bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\]
are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W {
        PA_W { w: self }
    }
}
