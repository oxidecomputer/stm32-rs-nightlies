#[doc = "Reader of register R2NONCER1"]
pub type R = crate::R<u32, super::R2NONCER1>;
#[doc = "Writer for register R2NONCER1"]
pub type W = crate::W<u32, super::R2NONCER1>;
#[doc = "Register R2NONCER1 `reset()`'s with value 0"]
impl crate::ResetValue for super::R2NONCER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REGx_NONCE`"]
pub type REGX_NONCE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `REGx_NONCE`"]
pub struct REGX_NONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_NONCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
    #[inline(always)]
    pub fn regx_nonce(&self) -> REGX_NONCE_R {
        REGX_NONCE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region nonce, bits \\[63:32\\]REGx_NONCE\\[63:32\\]"]
    #[inline(always)]
    pub fn regx_nonce(&mut self) -> REGX_NONCE_W {
        REGX_NONCE_W { w: self }
    }
}
