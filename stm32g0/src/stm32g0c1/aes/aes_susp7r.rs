#[doc = "Reader of register AES_SUSP7R"]
pub type R = crate::R<u32, super::AES_SUSP7R>;
#[doc = "Writer for register AES_SUSP7R"]
pub type W = crate::W<u32, super::AES_SUSP7R>;
#[doc = "Register AES_SUSP7R `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_SUSP7R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SUSP`"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend Upon suspend operation, this bitfield of every AES_SUSPxR register takes the value of one of internal AES registers."]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
}
