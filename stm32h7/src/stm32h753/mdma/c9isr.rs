#[doc = "Reader of register C9ISR"]
pub type R = crate::R<u32, super::C9ISR>;
#[doc = "Reader of field `TEIF9`"]
pub type TEIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF9`"]
pub type CTCIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF9`"]
pub type BRTIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF9`"]
pub type BTIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF9`"]
pub type TCIF9_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA9`"]
pub type CRQA9_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif9(&self) -> TEIF9_R {
        TEIF9_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif9(&self) -> CTCIF9_R {
        CTCIF9_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif9(&self) -> BRTIF9_R {
        BRTIF9_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif9(&self) -> BTIF9_R {
        BTIF9_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif9(&self) -> TCIF9_R {
        TCIF9_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa9(&self) -> CRQA9_R {
        CRQA9_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
