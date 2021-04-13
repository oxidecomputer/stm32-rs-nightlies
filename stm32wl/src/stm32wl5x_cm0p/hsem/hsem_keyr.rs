#[doc = "Reader of register HSEM_KEYR"]
pub type R = crate::R<u32, super::HSEM_KEYR>;
#[doc = "Writer for register HSEM_KEYR"]
pub type W = crate::W<u32, super::HSEM_KEYR>;
#[doc = "Register HSEM_KEYR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSEM_KEYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEY`"]
pub type KEY_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `KEY`"]
pub struct KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Semaphore Clear Key"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W {
        KEY_W { w: self }
    }
}
