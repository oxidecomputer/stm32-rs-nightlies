#[doc = "Reader of register MACPPSIR"]
pub type R = crate::R<u32, super::MACPPSIR>;
#[doc = "Writer for register MACPPSIR"]
pub type W = crate::W<u32, super::MACPPSIR>;
#[doc = "Register MACPPSIR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACPPSIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PPSINT0`"]
pub type PPSINT0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PPSINT0`"]
pub struct PPSINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PPSINT0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - PPS Output Signal Interval"]
    #[inline(always)]
    pub fn ppsint0(&self) -> PPSINT0_R {
        PPSINT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Output Signal Interval"]
    #[inline(always)]
    pub fn ppsint0(&mut self) -> PPSINT0_W {
        PPSINT0_W { w: self }
    }
}
