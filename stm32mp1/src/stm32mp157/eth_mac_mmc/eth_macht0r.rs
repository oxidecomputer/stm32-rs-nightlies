#[doc = "Reader of register ETH_MACHT0R"]
pub type R = crate::R<u32, super::ETH_MACHT0R>;
#[doc = "Writer for register ETH_MACHT0R"]
pub type W = crate::W<u32, super::ETH_MACHT0R>;
#[doc = "Register ETH_MACHT0R `reset()`'s with value 0"]
impl crate::ResetValue for super::ETH_MACHT0R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HT31T0`"]
pub type HT31T0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HT31T0`"]
pub struct HT31T0_W<'a> {
    w: &'a mut W,
}
impl<'a> HT31T0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W {
        HT31T0_W { w: self }
    }
}
