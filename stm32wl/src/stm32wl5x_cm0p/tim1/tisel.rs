#[doc = "Reader of register TISEL"]
pub type R = crate::R<u32, super::TISEL>;
#[doc = "Writer for register TISEL"]
pub type W = crate::W<u32, super::TISEL>;
#[doc = "Register TISEL `reset()`'s with value 0"]
impl crate::ResetValue for super::TISEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TI4SEL`"]
pub type TI4SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI4SEL`"]
pub struct TI4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI4SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `TI3SEL`"]
pub type TI3SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI3SEL`"]
pub struct TI3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `TI2SEL`"]
pub type TI2SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI2SEL`"]
pub struct TI2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TI1SEL`"]
pub type TI1SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TI1SEL`"]
pub struct TI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TI1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
    #[inline(always)]
    pub fn ti4sel(&self) -> TI4SEL_R {
        TI4SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    pub fn ti3sel(&self) -> TI3SEL_R {
        TI3SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - selects TI4\\[0\\]
to TI4\\[15\\]
input"]
    #[inline(always)]
    pub fn ti4sel(&mut self) -> TI4SEL_W {
        TI4SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\]
to TI3\\[15\\]
input"]
    #[inline(always)]
    pub fn ti3sel(&mut self) -> TI3SEL_W {
        TI3SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\]
to TI2\\[15\\]
input"]
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W {
        TI2SEL_W { w: self }
    }
    #[doc = "Bits 0:3 - selects TI1\\[0\\]
to TI1\\[15\\]
input"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W {
        TI1SEL_W { w: self }
    }
}
