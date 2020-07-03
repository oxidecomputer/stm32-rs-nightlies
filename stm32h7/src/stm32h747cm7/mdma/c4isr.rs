#[doc = "Reader of register C4ISR"]
pub type R = crate::R<u32, super::C4ISR>;
#[doc = "Reader of field `TEIF4`"]
pub type TEIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF4`"]
pub type CTCIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF4`"]
pub type BRTIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF4`"]
pub type BTIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF4`"]
pub type TCIF4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA4`"]
pub type CRQA4_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif4(&self) -> TEIF4_R {
        TEIF4_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif4(&self) -> CTCIF4_R {
        CTCIF4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif4(&self) -> BRTIF4_R {
        BRTIF4_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif4(&self) -> BTIF4_R {
        BTIF4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif4(&self) -> TCIF4_R {
        TCIF4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa4(&self) -> CRQA4_R {
        CRQA4_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
