#[doc = "Reader of register SUSP4R"]
pub type R = crate::R<u32, super::SUSP4R>;
#[doc = "Writer for register SUSP4R"]
pub type W = crate::W<u32, super::SUSP4R>;
#[doc = "Register SUSP4R `reset()`'s with value 0"]
impl crate::ResetValue for super::SUSP4R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_SUSP4R`"]
pub type AES_SUSP4R_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_SUSP4R`"]
pub struct AES_SUSP4R_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_SUSP4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES suspend register 4"]
    #[inline(always)]
    pub fn aes_susp4r(&self) -> AES_SUSP4R_R {
        AES_SUSP4R_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES suspend register 4"]
    #[inline(always)]
    pub fn aes_susp4r(&mut self) -> AES_SUSP4R_W {
        AES_SUSP4R_W { w: self }
    }
}
