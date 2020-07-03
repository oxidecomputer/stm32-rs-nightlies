#[doc = "Reader of register AHB1ENR"]
pub type R = crate::R<u32, super::AHB1ENR>;
#[doc = "Writer for register AHB1ENR"]
pub type W = crate::W<u32, super::AHB1ENR>;
#[doc = "Register AHB1ENR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB1ENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1 Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1EN_A {
    #[doc = "0: The selected clock is disabled"]
    DISABLED = 0,
    #[doc = "1: The selected clock is enabled"]
    ENABLED = 1,
}
impl From<DMA1EN_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1EN`"]
pub type DMA1EN_R = crate::R<bool, DMA1EN_A>;
impl DMA1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1EN_A {
        match self.bits {
            false => DMA1EN_A::DISABLED,
            true => DMA1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMA1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMA1EN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DMA1EN`"]
pub struct DMA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "DMA2 Clock Enable"]
pub type DMA2EN_A = DMA1EN_A;
#[doc = "Reader of field `DMA2EN`"]
pub type DMA2EN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `DMA2EN`"]
pub struct DMA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "ADC1/2 Peripheral Clocks Enable"]
pub type ADC12EN_A = DMA1EN_A;
#[doc = "Reader of field `ADC12EN`"]
pub type ADC12EN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `ADC12EN`"]
pub struct ADC12EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Ethernet MAC bus interface Clock Enable"]
pub type ETH1MACEN_A = DMA1EN_A;
#[doc = "Reader of field `ETH1MACEN`"]
pub type ETH1MACEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `ETH1MACEN`"]
pub struct ETH1MACEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1MACEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1MACEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Ethernet Transmission Clock Enable"]
pub type ETH1TXEN_A = DMA1EN_A;
#[doc = "Reader of field `ETH1TXEN`"]
pub type ETH1TXEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `ETH1TXEN`"]
pub struct ETH1TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1TXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1TXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Ethernet Reception Clock Enable"]
pub type ETH1RXEN_A = DMA1EN_A;
#[doc = "Reader of field `ETH1RXEN`"]
pub type ETH1RXEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `ETH1RXEN`"]
pub struct ETH1RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ETH1RXEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETH1RXEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Enable USB_PHY2 clocks"]
pub type USB2OTGHSULPIEN_A = DMA1EN_A;
#[doc = "Reader of field `USB2OTGHSULPIEN`"]
pub type USB2OTGHSULPIEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `USB2OTGHSULPIEN`"]
pub struct USB2OTGHSULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2OTGHSULPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2OTGHSULPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "USB1OTG Peripheral Clocks Enable"]
pub type USB1OTGHSEN_A = DMA1EN_A;
#[doc = "Reader of field `USB1OTGHSEN`"]
pub type USB1OTGHSEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `USB1OTGHSEN`"]
pub struct USB1OTGHSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGHSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGHSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "USB_PHY1 Clocks Enable"]
pub type USB1OTGHSULPIEN_A = DMA1EN_A;
#[doc = "Reader of field `USB1OTGHSULPIEN`"]
pub type USB1OTGHSULPIEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `USB1OTGHSULPIEN`"]
pub struct USB1OTGHSULPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB1OTGHSULPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB1OTGHSULPIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "USB2OTG Peripheral Clocks Enable"]
pub type USB2OTGHSEN_A = DMA1EN_A;
#[doc = "Reader of field `USB2OTGHSEN`"]
pub type USB2OTGHSEN_R = crate::R<bool, DMA1EN_A>;
#[doc = "Write proxy for field `USB2OTGHSEN`"]
pub struct USB2OTGHSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USB2OTGHSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB2OTGHSEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::DISABLED)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMA1EN_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> DMA1EN_R {
        DMA1EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> DMA2EN_R {
        DMA2EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> ADC12EN_R {
        ADC12EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&self) -> ETH1MACEN_R {
        ETH1MACEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&self) -> ETH1TXEN_R {
        ETH1TXEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&self) -> ETH1RXEN_R {
        ETH1RXEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Enable USB_PHY2 clocks"]
    #[inline(always)]
    pub fn usb2otghsulpien(&self) -> USB2OTGHSULPIEN_R {
        USB2OTGHSULPIEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otghsen(&self) -> USB1OTGHSEN_R {
        USB1OTGHSEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1otghsulpien(&self) -> USB1OTGHSULPIEN_R {
        USB1OTGHSULPIEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otghsen(&self) -> USB2OTGHSEN_R {
        USB2OTGHSEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> DMA1EN_W {
        DMA1EN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> DMA2EN_W {
        DMA2EN_W { w: self }
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> ADC12EN_W {
        ADC12EN_W { w: self }
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&mut self) -> ETH1MACEN_W {
        ETH1MACEN_W { w: self }
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&mut self) -> ETH1TXEN_W {
        ETH1TXEN_W { w: self }
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> ETH1RXEN_W {
        ETH1RXEN_W { w: self }
    }
    #[doc = "Bit 18 - Enable USB_PHY2 clocks"]
    #[inline(always)]
    pub fn usb2otghsulpien(&mut self) -> USB2OTGHSULPIEN_W {
        USB2OTGHSULPIEN_W { w: self }
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otghsen(&mut self) -> USB1OTGHSEN_W {
        USB1OTGHSEN_W { w: self }
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1otghsulpien(&mut self) -> USB1OTGHSULPIEN_W {
        USB1OTGHSULPIEN_W { w: self }
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otghsen(&mut self) -> USB2OTGHSEN_W {
        USB2OTGHSEN_W { w: self }
    }
}
