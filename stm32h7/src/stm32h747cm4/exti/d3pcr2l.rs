#[doc = "Reader of register D3PCR2L"]
pub type R = crate::R<u32, super::D3PCR2L>;
#[doc = "Writer for register D3PCR2L"]
pub type W = crate::W<u32, super::D3PCR2L>;
#[doc = "Register D3PCR2L `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR2L {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCS35`"]
pub type PCS35_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS35`"]
pub struct PCS35_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS35_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCS34`"]
pub type PCS34_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS34`"]
pub struct PCS34_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCS41`"]
pub type PCS41_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS41`"]
pub struct PCS41_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS41_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&self) -> PCS35_R {
        PCS35_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&self) -> PCS34_R {
        PCS34_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&self) -> PCS41_R {
        PCS41_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs35(&mut self) -> PCS35_W {
        PCS35_W { w: self }
    }
    #[doc = "Bits 4:5 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs34(&mut self) -> PCS34_W {
        PCS34_W { w: self }
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+64)/2)"]
    #[inline(always)]
    pub fn pcs41(&mut self) -> PCS41_W {
        PCS41_W { w: self }
    }
}
