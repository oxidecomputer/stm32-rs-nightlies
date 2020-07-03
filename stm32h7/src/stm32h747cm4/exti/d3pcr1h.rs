#[doc = "Reader of register D3PCR1H"]
pub type R = crate::R<u32, super::D3PCR1H>;
#[doc = "Writer for register D3PCR1H"]
pub type W = crate::W<u32, super::D3PCR1H>;
#[doc = "Register D3PCR1H `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR1H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCS19`"]
pub type PCS19_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS19`"]
pub struct PCS19_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCS20`"]
pub type PCS20_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS20`"]
pub struct PCS20_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCS21`"]
pub type PCS21_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS21`"]
pub struct PCS21_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `PCS25`"]
pub type PCS25_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS25`"]
pub struct PCS25_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS25_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&self) -> PCS19_R {
        PCS19_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&self) -> PCS20_R {
        PCS20_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&self) -> PCS21_R {
        PCS21_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&self) -> PCS25_R {
        PCS25_R::new(((self.bits >> 18) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs19(&mut self) -> PCS19_W {
        PCS19_W { w: self }
    }
    #[doc = "Bits 8:9 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs20(&mut self) -> PCS20_W {
        PCS20_W { w: self }
    }
    #[doc = "Bits 10:11 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs21(&mut self) -> PCS21_W {
        PCS21_W { w: self }
    }
    #[doc = "Bits 18:19 - D3 Pending request clear input signal selection on Event input x = truncate ((n+32)/2)"]
    #[inline(always)]
    pub fn pcs25(&mut self) -> PCS25_W {
        PCS25_W { w: self }
    }
}
