#[doc = "Reader of register C10ISR"]
pub type R = crate::R<u32, super::C10ISR>;
#[doc = "Reader of field `TEIF10`"]
pub type TEIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF10`"]
pub type CTCIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF10`"]
pub type BRTIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF10`"]
pub type BTIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF10`"]
pub type TCIF10_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA10`"]
pub type CRQA10_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif10(&self) -> TEIF10_R {
        TEIF10_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif10(&self) -> CTCIF10_R {
        CTCIF10_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif10(&self) -> BRTIF10_R {
        BRTIF10_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif10(&self) -> BTIF10_R {
        BTIF10_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif10(&self) -> TCIF10_R {
        TCIF10_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa10(&self) -> CRQA10_R {
        CRQA10_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
