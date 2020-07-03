#[doc = "Reader of register C0ISR"]
pub type R = crate::R<u32, super::C0ISR>;
#[doc = "Reader of field `TEIF0`"]
pub type TEIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF0`"]
pub type CTCIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF0`"]
pub type BRTIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF0`"]
pub type BTIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF0`"]
pub type TCIF0_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA0`"]
pub type CRQA0_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif0(&self) -> TEIF0_R {
        TEIF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif0(&self) -> CTCIF0_R {
        CTCIF0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif0(&self) -> BRTIF0_R {
        BRTIF0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif0(&self) -> BTIF0_R {
        BTIF0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif0(&self) -> TCIF0_R {
        TCIF0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa0(&self) -> CRQA0_R {
        CRQA0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
