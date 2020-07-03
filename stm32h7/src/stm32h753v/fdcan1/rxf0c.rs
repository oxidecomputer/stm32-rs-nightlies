#[doc = "Reader of register RXF0C"]
pub type R = crate::R<u32, super::RXF0C>;
#[doc = "Writer for register RXF0C"]
pub type W = crate::W<u32, super::RXF0C>;
#[doc = "Register RXF0C `reset()`'s with value 0"]
impl crate::ResetValue for super::RXF0C {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `F0SA`"]
pub type F0SA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `F0SA`"]
pub struct F0SA_W<'a> {
    w: &'a mut W,
}
impl<'a> F0SA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 2)) | (((value as u32) & 0x3fff) << 2);
        self.w
    }
}
#[doc = "Reader of field `F0S`"]
pub type F0S_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0S`"]
pub struct F0S_W<'a> {
    w: &'a mut W,
}
impl<'a> F0S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `F0WM`"]
pub type F0WM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `F0WM`"]
pub struct F0WM_W<'a> {
    w: &'a mut W,
}
impl<'a> F0WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0SA_R {
        F0SA_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0S_R {
        F0S_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0WM_R {
        F0WM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0SA_W {
        F0SA_W { w: self }
    }
    #[doc = "Bits 16:23 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&mut self) -> F0S_W {
        F0S_W { w: self }
    }
    #[doc = "Bits 24:31 - FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0WM_W {
        F0WM_W { w: self }
    }
}