#[doc = "Reader of register C8ISR"]
pub type R = crate::R<u32, super::C8ISR>;
#[doc = "Reader of field `TEIF8`"]
pub type TEIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CTCIF8`"]
pub type CTCIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BRTIF8`"]
pub type BRTIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `BTIF8`"]
pub type BTIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCIF8`"]
pub type TCIF8_R = crate::R<bool, bool>;
#[doc = "Reader of field `CRQA8`"]
pub type CRQA8_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif8(&self) -> TEIF8_R {
        TEIF8_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif8(&self) -> CTCIF8_R {
        CTCIF8_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif8(&self) -> BRTIF8_R {
        BRTIF8_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif8(&self) -> BTIF8_R {
        BTIF8_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif8(&self) -> TCIF8_R {
        TCIF8_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa8(&self) -> CRQA8_R {
        CRQA8_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
