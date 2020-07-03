#[doc = "Reader of register D3PCR3H"]
pub type R = crate::R<u32, super::D3PCR3H>;
#[doc = "Writer for register D3PCR3H"]
pub type W = crate::W<u32, super::D3PCR3H>;
#[doc = "Register D3PCR3H `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR3H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCS88`"]
pub type PCS88_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS88`"]
pub struct PCS88_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS88_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&self) -> PCS88_R {
        PCS88_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x= truncate N+160/2"]
    #[inline(always)]
    pub fn pcs88(&mut self) -> PCS88_W {
        PCS88_W { w: self }
    }
}
