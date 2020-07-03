#[doc = "Writer for register TXDR"]
pub type W = crate::W<u32, super::TXDR>;
#[doc = "Register TXDR `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TXD`"]
pub struct TXD_W<'a> {
    w: &'a mut W,
}
impl<'a> TXD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Tx Data register. TXD is a write-only register containing the data byte to be transmitted. Note: TXD must be written when TXSTART=1"]
    #[inline(always)]
    pub fn txd(&mut self) -> TXD_W {
        TXD_W { w: self }
    }
}
