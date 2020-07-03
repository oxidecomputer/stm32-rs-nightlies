#[doc = "Reader of register CFGR3"]
pub type R = crate::R<u32, super::CFGR3>;
#[doc = "Writer for register CFGR3"]
pub type W = crate::W<u32, super::CFGR3>;
#[doc = "Register CFGR3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C1_RX_DMA_RMP`"]
pub type I2C1_RX_DMA_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1_RX_DMA_RMP`"]
pub struct I2C1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RX_DMA_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPI1_TX_DMA_RMP`"]
pub type SPI1_TX_DMA_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_TX_DMA_RMP`"]
pub struct SPI1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_TX_DMA_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `SPI1_RX_DMA_RMP`"]
pub type SPI1_RX_DMA_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPI1_RX_DMA_RMP`"]
pub struct SPI1_RX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI1_RX_DMA_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `I2C1_TX_DMA_RMP`"]
pub type I2C1_TX_DMA_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2C1_TX_DMA_RMP`"]
pub struct I2C1_TX_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_TX_DMA_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `ADC2_DMA_RMP`"]
pub type ADC2_DMA_RMP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADC2_DMA_RMP`"]
pub struct ADC2_DMA_RMP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_DMA_RMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&self) -> I2C1_RX_DMA_RMP_R {
        I2C1_RX_DMA_RMP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&self) -> SPI1_TX_DMA_RMP_R {
        SPI1_TX_DMA_RMP_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&self) -> SPI1_RX_DMA_RMP_R {
        SPI1_RX_DMA_RMP_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&self) -> I2C1_TX_DMA_RMP_R {
        I2C1_TX_DMA_RMP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&mut self) -> I2C1_RX_DMA_RMP_W {
        I2C1_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&mut self) -> SPI1_TX_DMA_RMP_W {
        SPI1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&mut self) -> SPI1_RX_DMA_RMP_W {
        SPI1_RX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&mut self) -> I2C1_TX_DMA_RMP_W {
        I2C1_TX_DMA_RMP_W { w: self }
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W {
        ADC2_DMA_RMP_W { w: self }
    }
}
