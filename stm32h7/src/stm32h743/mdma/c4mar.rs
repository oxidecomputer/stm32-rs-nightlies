#[doc = "Reader of register C4MAR"]
pub type R = crate::R<u32, super::C4MAR>;
#[doc = "Writer for register C4MAR"]
pub type W = crate::W<u32, super::C4MAR>;
#[doc = "Register C4MAR `reset()`'s with value 0"]
impl crate::ResetValue for super::C4MAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAR`"]
pub type MAR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MAR`"]
pub struct MAR_W<'a> {
    w: &'a mut W,
}
impl<'a> MAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&self) -> MAR_R {
        MAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&mut self) -> MAR_W {
        MAR_W { w: self }
    }
}
