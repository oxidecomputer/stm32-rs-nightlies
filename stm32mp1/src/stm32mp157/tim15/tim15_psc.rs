#[doc = "Reader of register TIM15_PSC"]
pub type R = crate::R<u16, super::TIM15_PSC>;
#[doc = "Writer for register TIM15_PSC"]
pub type W = crate::W<u16, super::TIM15_PSC>;
#[doc = "Register TIM15_PSC `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM15_PSC {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
