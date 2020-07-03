#[doc = "Reader of register C2ISR"]
pub type R = crate::R<u32, super::C2ISR>;
#[doc = "Reader of field `TEIF2`"]
pub type TEIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF2`"]
pub type CTCIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF2`"]
pub type BRTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF2`"]
pub type BTIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF2`"]
pub type TCIF2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA2`"]
pub type CRQA2_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif2(&self) -> TEIF2_R {
        TEIF2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif2(&self) -> CTCIF2_R {
        CTCIF2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif2(&self) -> BRTIF2_R {
        BRTIF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif2(&self) -> BTIF2_R {
        BTIF2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif2(&self) -> TCIF2_R {
        TCIF2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa2(&self) -> CRQA2_R {
        CRQA2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
