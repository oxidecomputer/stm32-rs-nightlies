#[doc = "Writer for register K3LR"]
pub type W = crate::W<u32, super::K3LR>;
#[doc = "Register K3LR `reset()`'s with value 0"]
impl crate::ResetValue for super::K3LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `K`"]
pub struct K_W<'a> {
    w: &'a mut W,
}
impl<'a> K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - K32"]
    #[inline(always)]
    pub fn k(&mut self) -> K_W {
        K_W { w: self }
    }
}
