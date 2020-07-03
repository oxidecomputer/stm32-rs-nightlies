#[doc = "Reader of register C13ISR"]
pub type R = crate::R<u32, super::C13ISR>;
#[doc = "Reader of field `TEIF13`"]
pub type TEIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF13`"]
pub type CTCIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF13`"]
pub type BRTIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF13`"]
pub type BTIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF13`"]
pub type TCIF13_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA13`"]
pub type CRQA13_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif13(&self) -> TEIF13_R {
        TEIF13_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif13(&self) -> CTCIF13_R {
        CTCIF13_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif13(&self) -> BRTIF13_R {
        BRTIF13_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif13(&self) -> BTIF13_R {
        BTIF13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif13(&self) -> TCIF13_R {
        TCIF13_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa13(&self) -> CRQA13_R {
        CRQA13_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
