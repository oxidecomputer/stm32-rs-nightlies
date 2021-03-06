#[doc = "Reader of register FGMAR"]
pub type R = crate::R<u32, super::FGMAR>;
#[doc = "Writer for register FGMAR"]
pub type W = crate::W<u32, super::FGMAR>;
#[doc = "Register FGMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::FGMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MA`"]
pub type MA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MA`"]
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Address of the data used for the foreground image. This register can only be written when data transfers are disabled. Once the data transfer has started, this register is read-only. The address alignment must match the image format selected e.g. a 32-bit per pixel format must be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned and a 4-bit per pixel format must be 8-bit aligned."]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
}
