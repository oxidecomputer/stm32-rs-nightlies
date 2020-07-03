#[doc = "Reader of register C15ISR"]
pub type R = crate::R<u32, super::C15ISR>;
#[doc = "Reader of field `TEIF15`"]
pub type TEIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF15`"]
pub type CTCIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF15`"]
pub type BRTIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF15`"]
pub type BTIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF15`"]
pub type TCIF15_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA15`"]
pub type CRQA15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif15(&self) -> TEIF15_R {
        TEIF15_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif15(&self) -> CTCIF15_R {
        CTCIF15_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif15(&self) -> BRTIF15_R {
        BRTIF15_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif15(&self) -> BTIF15_R {
        BTIF15_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif15(&self) -> TCIF15_R {
        TCIF15_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa15(&self) -> CRQA15_R {
        CRQA15_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
