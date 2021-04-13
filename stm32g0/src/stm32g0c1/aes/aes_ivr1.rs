#[doc = "Reader of register AES_IVR1"]
pub type R = crate::R<u32, super::AES_IVR1>;
#[doc = "Writer for register AES_IVR1"]
pub type W = crate::W<u32, super::AES_IVR1>;
#[doc = "Register AES_IVR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AES_IVR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IVI`"]
pub type IVI_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `IVI`"]
pub struct IVI_W<'a> {
    w: &'a mut W,
}
impl<'a> IVI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[63:32\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    pub fn ivi(&self) -> IVI_R {
        IVI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Initialization vector input, bits \\[63:32\\]
Refer to the AES_IVR0 register for description of the IVI\\[128:0\\]
bitfield."]
    #[inline(always)]
    pub fn ivi(&mut self) -> IVI_W {
        IVI_W { w: self }
    }
}
