#[doc = "Reader of register HTR1"]
pub type R = crate::R<u32, super::HTR1>;
#[doc = "Writer for register HTR1"]
pub type W = crate::W<u32, super::HTR1>;
#[doc = "Register HTR1 `reset()`'s with value 0x0fff_0000"]
impl crate::ResetValue for super::HTR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fff_0000
    }
}
#[doc = "Reader of field `HTR1`"]
pub type HTR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HTR1`"]
pub struct HTR1_W<'a> {
    w: &'a mut W,
}
impl<'a> HTR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn htr1(&self) -> HTR1_R {
        HTR1_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn htr1(&mut self) -> HTR1_W {
        HTR1_W { w: self }
    }
}
