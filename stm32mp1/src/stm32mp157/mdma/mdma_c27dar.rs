#[doc = "Reader of register MDMA_C27DAR"]
pub type R = crate::R<u32, super::MDMA_C27DAR>;
#[doc = "Writer for register MDMA_C27DAR"]
pub type W = crate::W<u32, super::MDMA_C27DAR>;
#[doc = "Register MDMA_C27DAR `reset()`'s with value 0"]
impl crate::ResetValue for super::MDMA_C27DAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAR`"]
pub type DAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DAR`"]
pub struct DAR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W { w: self }
    }
}