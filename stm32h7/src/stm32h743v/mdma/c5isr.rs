#[doc = "Reader of register C5ISR"]
pub type R = crate::R<u32, super::C5ISR>;
#[doc = "Reader of field `TEIF5`"]
pub type TEIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF5`"]
pub type CTCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF5`"]
pub type BRTIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF5`"]
pub type BTIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF5`"]
pub type TCIF5_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA5`"]
pub type CRQA5_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif5(&self) -> TEIF5_R {
        TEIF5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif5(&self) -> CTCIF5_R {
        CTCIF5_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif5(&self) -> BRTIF5_R {
        BRTIF5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif5(&self) -> BTIF5_R {
        BTIF5_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif5(&self) -> TCIF5_R {
        TCIF5_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa5(&self) -> CRQA5_R {
        CRQA5_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
