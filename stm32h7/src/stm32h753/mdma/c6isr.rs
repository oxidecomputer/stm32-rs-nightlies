#[doc = "Reader of register C6ISR"]
pub type R = crate::R<u32, super::C6ISR>;
#[doc = "Reader of field `TEIF6`"]
pub type TEIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF6`"]
pub type CTCIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF6`"]
pub type BRTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF6`"]
pub type BTIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF6`"]
pub type TCIF6_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA6`"]
pub type CRQA6_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif6(&self) -> TEIF6_R {
        TEIF6_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif6(&self) -> CTCIF6_R {
        CTCIF6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif6(&self) -> BRTIF6_R {
        BRTIF6_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif6(&self) -> BTIF6_R {
        BTIF6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif6(&self) -> TCIF6_R {
        TCIF6_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa6(&self) -> CRQA6_R {
        CRQA6_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
