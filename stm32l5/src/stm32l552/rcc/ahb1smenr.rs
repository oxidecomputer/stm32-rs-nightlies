#[doc = "Reader of register AHB1SMENR"]
pub type R = crate::R<u32, super::AHB1SMENR>;
#[doc = "Writer for register AHB1SMENR"]
pub type W = crate::W<u32, super::AHB1SMENR>;
#[doc = "Register AHB1SMENR `reset()`'s with value 0x00c1_1307"]
impl crate::ResetValue for super::AHB1SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00c1_1307
    }
}
#[doc = "Reader of field `DMA1SMEN`"]
pub type DMA1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1SMEN`"]
pub struct DMA1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SMEN_W<'a> {
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
#[doc = "Reader of field `DMA2SMEN`"]
pub type DMA2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2SMEN`"]
pub struct DMA2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2SMEN_W<'a> {
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
#[doc = "Reader of field `DMAMUX1SMEN`"]
pub type DMAMUX1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1SMEN`"]
pub struct DMAMUX1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `FLASHSMEN`"]
pub type FLASHSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHSMEN`"]
pub struct FLASHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SRAM1SMEN`"]
pub type SRAM1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1SMEN`"]
pub struct SRAM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CRCSMEN`"]
pub type CRCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCSMEN`"]
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TSCSMEN`"]
pub type TSCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSCSMEN`"]
pub struct TSCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSCSMEN_W<'a> {
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
#[doc = "Reader of field `GTZCSMEN`"]
pub type GTZCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTZCSMEN`"]
pub struct GTZCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GTZCSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ICACHESMEN`"]
pub type ICACHESMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ICACHESMEN`"]
pub struct ICACHESMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ICACHESMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&self) -> TSCSMEN_R {
        TSCSMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 22 - GTZCSMEN"]
    #[inline(always)]
    pub fn gtzcsmen(&self) -> GTZCSMEN_R {
        GTZCSMEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ICACHESMEN"]
    #[inline(always)]
    pub fn icachesmen(&self) -> ICACHESMEN_R {
        ICACHESMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W {
        DMA1SMEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W {
        DMA2SMEN_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUX clock enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W {
        DMAMUX1SMEN_W { w: self }
    }
    #[doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W {
        FLASHSMEN_W { w: self }
    }
    #[doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W {
        SRAM1SMEN_W { w: self }
    }
    #[doc = "Bit 12 - CRCSMEN"]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    #[doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn tscsmen(&mut self) -> TSCSMEN_W {
        TSCSMEN_W { w: self }
    }
    #[doc = "Bit 22 - GTZCSMEN"]
    #[inline(always)]
    pub fn gtzcsmen(&mut self) -> GTZCSMEN_W {
        GTZCSMEN_W { w: self }
    }
    #[doc = "Bit 23 - ICACHESMEN"]
    #[inline(always)]
    pub fn icachesmen(&mut self) -> ICACHESMEN_W {
        ICACHESMEN_W { w: self }
    }
}
