#[doc = "Reader of register DVR"]
pub type R = crate::R<u32, super::DVR>;
#[doc = "Writer for register DVR"]
pub type W = crate::W<u32, super::DVR>;
#[doc = "Register DVR `reset()`'s with value 0"]
impl crate::ResetValue for super::DVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DV`"]
pub type DV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DV`"]
pub struct DV_W<'a> {
    w: &'a mut W,
}
impl<'a> DV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W {
        DV_W { w: self }
    }
}
