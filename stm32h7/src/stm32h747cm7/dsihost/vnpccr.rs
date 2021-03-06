#[doc = "Reader of register VNPCCR"]
pub type R = crate::R<u32, super::VNPCCR>;
#[doc = "Writer for register VNPCCR"]
pub type W = crate::W<u32, super::VNPCCR>;
#[doc = "Register VNPCCR `reset()`'s with value 0"]
impl crate::ResetValue for super::VNPCCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NPSIZE`"]
pub type NPSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPSIZE`"]
pub struct NPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Null packet size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Null packet size"]
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W {
        NPSIZE_W { w: self }
    }
}
