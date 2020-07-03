#[doc = "Reader of register C7ISR"]
pub type R = crate::R<u32, super::C7ISR>;
#[doc = "Reader of field `TEIF7`"]
pub type TEIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF7`"]
pub type CTCIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF7`"]
pub type BRTIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF7`"]
pub type BTIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF7`"]
pub type TCIF7_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA7`"]
pub type CRQA7_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif7(&self) -> TEIF7_R {
        TEIF7_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif7(&self) -> CTCIF7_R {
        CTCIF7_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif7(&self) -> BRTIF7_R {
        BRTIF7_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif7(&self) -> BTIF7_R {
        BTIF7_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif7(&self) -> TCIF7_R {
        TCIF7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa7(&self) -> CRQA7_R {
        CRQA7_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
