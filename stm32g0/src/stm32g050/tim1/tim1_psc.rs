#[doc = "Reader of register TIM1_PSC"]
pub type R = crate::R<u32, super::TIM1_PSC>;
#[doc = "Writer for register TIM1_PSC"]
pub type W = crate::W<u32, super::TIM1_PSC>;
#[doc = "Register TIM1_PSC `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1_PSC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PSC`"]
pub type PSC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PSC`"]
pub struct PSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in â\u{80}\u{9c}reset modeâ\u{80}\u{9d})."]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Prescaler value The counter clock frequency (CK_CNT) is equal to fCK_PSC / (PSC\\[15:0\\]
+ 1). PSC contains the value to be loaded in the active prescaler register at each update event (including when the counter is cleared through UG bit of TIMx_EGR register or through trigger controller when configured in â\u{80}\u{9c}reset modeâ\u{80}\u{9d})."]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W {
        PSC_W { w: self }
    }
}
