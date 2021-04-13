#[doc = "Reader of register TIM1_DMAR"]
pub type R = crate::R<u32, super::TIM1_DMAR>;
#[doc = "Writer for register TIM1_DMAR"]
pub type W = crate::W<u32, super::TIM1_DMAR>;
#[doc = "Register TIM1_DMAR `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM1_DMAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DMAB`"]
pub type DMAB_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DMAB`"]
pub struct DMAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA register for burst accesses A read or write operation to the DMAR register accesses the register located at the address (TIMx_CR1 address) + (DBA + DMA index) x 4 where TIMx_CR1 address is the address of the control register 1, DBA is the DMA base address configured in TIMx_DCR register, DMA index is automatically controlled by the DMA transfer, and ranges from 0 to DBL (DBL configured in TIMx_DCR)."]
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W {
        DMAB_W { w: self }
    }
}
