#[doc = "Reader of register AR"]
pub type R = crate::R<u32, super::AR>;
#[doc = "Writer for register AR"]
pub type W = crate::W<u32, super::AR>;
#[doc = "Register AR `reset()`'s with value 0"]
impl crate::ResetValue for super::AR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADRESS`"]
pub type ADRESS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADRESS`"]
pub struct ADRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> ADRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Adress"]
    #[inline(always)]
    pub fn adress(&self) -> ADRESS_R {
        ADRESS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Adress"]
    #[inline(always)]
    pub fn adress(&mut self) -> ADRESS_W {
        ADRESS_W { w: self }
    }
}
