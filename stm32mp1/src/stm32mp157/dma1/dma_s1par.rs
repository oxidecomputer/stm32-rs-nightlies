#[doc = "Reader of register DMA_S1PAR"]
pub type R = crate::R<u32, super::DMA_S1PAR>;
#[doc = "Writer for register DMA_S1PAR"]
pub type W = crate::W<u32, super::DMA_S1PAR>;
#[doc = "Register DMA_S1PAR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMA_S1PAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PAR`"]
pub type PAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PAR`"]
pub struct PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W {
        PAR_W { w: self }
    }
}
