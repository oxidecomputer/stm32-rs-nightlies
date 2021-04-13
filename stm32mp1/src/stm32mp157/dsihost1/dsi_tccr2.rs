#[doc = "Reader of register DSI_TCCR2"]
pub type R = crate::R<u32, super::DSI_TCCR2>;
#[doc = "Writer for register DSI_TCCR2"]
pub type W = crate::W<u32, super::DSI_TCCR2>;
#[doc = "Register DSI_TCCR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_TCCR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LPRD_TOCNT`"]
pub type LPRD_TOCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LPRD_TOCNT`"]
pub struct LPRD_TOCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LPRD_TOCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - LPRD_TOCNT"]
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LPRD_TOCNT_R {
        LPRD_TOCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPRD_TOCNT"]
    #[inline(always)]
    pub fn lprd_tocnt(&mut self) -> LPRD_TOCNT_W {
        LPRD_TOCNT_W { w: self }
    }
}
