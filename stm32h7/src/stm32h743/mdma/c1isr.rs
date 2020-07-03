#[doc = "Reader of register C1ISR"]
pub type R = crate::R<u32, super::C1ISR>;
#[doc = "Reader of field `TEIF1`"]
pub type TEIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF1`"]
pub type CTCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF1`"]
pub type BRTIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF1`"]
pub type BTIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF1`"]
pub type TCIF1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA1`"]
pub type CRQA1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif1(&self) -> TEIF1_R {
        TEIF1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif1(&self) -> CTCIF1_R {
        CTCIF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif1(&self) -> BRTIF1_R {
        BRTIF1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif1(&self) -> BTIF1_R {
        BTIF1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif1(&self) -> TCIF1_R {
        TCIF1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa1(&self) -> CRQA1_R {
        CRQA1_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
