#[doc = "Reader of register KEYR5"]
pub type R = crate::R<u32, super::KEYR5>;
#[doc = "Writer for register KEYR5"]
pub type W = crate::W<u32, super::KEYR5>;
#[doc = "Register KEYR5 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYR5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES_KEYR5`"]
pub type AES_KEYR5_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `AES_KEYR5`"]
pub struct AES_KEYR5_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_KEYR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[191:160\\])"]
    #[inline(always)]
    pub fn aes_keyr5(&self) -> AES_KEYR5_R {
        AES_KEYR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES key register (MSB key \\[191:160\\])"]
    #[inline(always)]
    pub fn aes_keyr5(&mut self) -> AES_KEYR5_W {
        AES_KEYR5_W { w: self }
    }
}
