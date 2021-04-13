#[doc = "Reader of register ARR"]
pub type R = crate::R<u16, super::ARR>;
#[doc = "Writer for register ARR"]
pub type W = crate::W<u16, super::ARR>;
#[doc = "Register ARR `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::ARR {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `ARR`"]
pub type ARR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ARR`"]
pub struct ARR_W<'a> {
    w: &'a mut W,
}
impl<'a> ARR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Auto-reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W {
        ARR_W { w: self }
    }
}
