#[doc = "Reader of register C14ISR"]
pub type R = crate::R<u32, super::C14ISR>;
#[doc = "Reader of field `TEIF14`"]
pub type TEIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF14`"]
pub type CTCIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF14`"]
pub type BRTIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF14`"]
pub type BTIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF14`"]
pub type TCIF14_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA14`"]
pub type CRQA14_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif14(&self) -> TEIF14_R {
        TEIF14_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif14(&self) -> CTCIF14_R {
        CTCIF14_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif14(&self) -> BRTIF14_R {
        BRTIF14_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif14(&self) -> BTIF14_R {
        BTIF14_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif14(&self) -> TCIF14_R {
        TCIF14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa14(&self) -> CRQA14_R {
        CRQA14_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
