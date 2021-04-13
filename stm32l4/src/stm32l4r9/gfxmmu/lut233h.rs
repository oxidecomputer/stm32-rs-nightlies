#[doc = "Reader of register LUT233H"]
pub type R = crate::R<u32, super::LUT233H>;
#[doc = "Writer for register LUT233H"]
pub type W = crate::W<u32, super::LUT233H>;
#[doc = "Register LUT233H `reset()`'s with value 0"]
impl crate::ResetValue for super::LUT233H {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LO`"]
pub type LO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LO`"]
pub struct LO_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0003_ffff << 4)) | (((value as u32) & 0x0003_ffff) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W {
        LO_W { w: self }
    }
}
