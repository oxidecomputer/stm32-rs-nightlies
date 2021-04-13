#[doc = "Reader of register ETH_MACTSIACR"]
pub type R = crate::R<u32, super::ETH_MACTSIACR>;
#[doc = "Writer for register ETH_MACTSIACR"]
pub type W = crate::W<u32, super::ETH_MACTSIACR>;
#[doc = "Register ETH_MACTSIACR `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACTSIACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OSTIAC`"]
pub type OSTIAC_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OSTIAC`"]
pub struct OSTIAC_W<'a> {
    w: &'a mut W,
}
impl<'a> OSTIAC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&self) -> OSTIAC_R {
        OSTIAC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&mut self) -> OSTIAC_W {
        OSTIAC_W { w: self }
    }
}
