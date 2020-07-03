#[doc = "Reader of register D3PCR2H"]
pub type R = crate::R<u32, super::D3PCR2H>;
#[doc = "Writer for register D3PCR2H"]
pub type W = crate::W<u32, super::D3PCR2H>;
#[doc = "Register D3PCR2H `reset()`'s with value 0"]
impl crate::ResetValue for super::D3PCR2H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PCS48`"]
pub type PCS48_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS48`"]
pub struct PCS48_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS48_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PCS49`"]
pub type PCS49_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS49`"]
pub struct PCS49_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS49_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `PCS50`"]
pub type PCS50_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS50`"]
pub struct PCS50_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS50_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PCS51`"]
pub type PCS51_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS51`"]
pub struct PCS51_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS51_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `PCS52`"]
pub type PCS52_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS52`"]
pub struct PCS52_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS52_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCS53`"]
pub type PCS53_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCS53`"]
pub struct PCS53_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS53_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&self) -> PCS48_R {
        PCS48_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&self) -> PCS49_R {
        PCS49_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&self) -> PCS50_R {
        PCS50_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&self) -> PCS51_R {
        PCS51_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&self) -> PCS52_R {
        PCS52_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&self) -> PCS53_R {
        PCS53_R::new(((self.bits >> 10) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs48(&mut self) -> PCS48_W {
        PCS48_W { w: self }
    }
    #[doc = "Bits 2:3 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs49(&mut self) -> PCS49_W {
        PCS49_W { w: self }
    }
    #[doc = "Bits 4:5 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs50(&mut self) -> PCS50_W {
        PCS50_W { w: self }
    }
    #[doc = "Bits 6:7 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs51(&mut self) -> PCS51_W {
        PCS51_W { w: self }
    }
    #[doc = "Bits 8:9 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs52(&mut self) -> PCS52_W {
        PCS52_W { w: self }
    }
    #[doc = "Bits 10:11 - Pending request clear input signal selection on Event input x= truncate ((n+96)/2)"]
    #[inline(always)]
    pub fn pcs53(&mut self) -> PCS53_W {
        PCS53_W { w: self }
    }
}
