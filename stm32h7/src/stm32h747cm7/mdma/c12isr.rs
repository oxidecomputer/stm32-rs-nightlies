#[doc = "Reader of register C12ISR"]
pub type R = crate::R<u32, super::C12ISR>;
#[doc = "Reader of field `TEIF12`"]
pub type TEIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF12`"]
pub type CTCIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF12`"]
pub type BRTIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF12`"]
pub type BTIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF12`"]
pub type TCIF12_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA12`"]
pub type CRQA12_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif12(&self) -> TEIF12_R {
        TEIF12_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif12(&self) -> CTCIF12_R {
        CTCIF12_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif12(&self) -> BRTIF12_R {
        BRTIF12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif12(&self) -> BTIF12_R {
        BTIF12_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif12(&self) -> TCIF12_R {
        TCIF12_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa12(&self) -> CRQA12_R {
        CRQA12_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
