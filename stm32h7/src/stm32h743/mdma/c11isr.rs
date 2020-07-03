#[doc = "Reader of register C11ISR"]
pub type R = crate::R<u32, super::C11ISR>;
#[doc = "Reader of field `TEIF11`"]
pub type TEIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF11`"]
pub type CTCIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF11`"]
pub type BRTIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF11`"]
pub type BTIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF11`"]
pub type TCIF11_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA11`"]
pub type CRQA11_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif11(&self) -> TEIF11_R {
        TEIF11_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif11(&self) -> CTCIF11_R {
        CTCIF11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif11(&self) -> BRTIF11_R {
        BRTIF11_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif11(&self) -> BTIF11_R {
        BTIF11_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif11(&self) -> TCIF11_R {
        TCIF11_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa11(&self) -> CRQA11_R {
        CRQA11_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
