#[doc = "Reader of register C3ISR"]
pub type R = crate::R<u32, super::C3ISR>;
#[doc = "Reader of field `TEIF3`"]
pub type TEIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF3`"]
pub type CTCIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF3`"]
pub type BRTIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF3`"]
pub type BTIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF3`"]
pub type TCIF3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA3`"]
pub type CRQA3_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif3(&self) -> TEIF3_R {
        TEIF3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif3(&self) -> CTCIF3_R {
        CTCIF3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif3(&self) -> BRTIF3_R {
        BRTIF3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif3(&self) -> BTIF3_R {
        BTIF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif3(&self) -> TCIF3_R {
        TCIF3_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa3(&self) -> CRQA3_R {
        CRQA3_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
