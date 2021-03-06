#[doc = "Reader of register IV0LR"]
pub type R = crate::R<u32, super::IV0LR>;
#[doc = "Writer for register IV0LR"]
pub type W = crate::W<u32, super::IV0LR>;
#[doc = "Register IV0LR `reset()`'s with value 0"]
impl crate::ResetValue for super::IV0LR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IV`"]
pub type IV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IV`"]
pub struct IV_W<'a> {
    w: &'a mut W,
}
impl<'a> IV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    pub fn iv(&self) -> IV_R {
        IV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - IV31"]
    #[inline(always)]
    pub fn iv(&mut self) -> IV_W {
        IV_W { w: self }
    }
}
