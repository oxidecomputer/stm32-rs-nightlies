#[doc = "Reader of register AHB1RSTR"]
pub type R = crate::R<u32, super::AHB1RSTR>;
#[doc = "Writer for register AHB1RSTR"]
pub type W = crate::W<u32, super::AHB1RSTR>;
#[doc = "Register AHB1RSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::AHB1RSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RST_A {
    #[doc = "1: Reset the selected module"]
    RESET = 1,
}
impl From<DMA1RST_A> for bool {
    #[inline(always)]
    fn from(variant: DMA1RST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DMA1RST`"]
pub type DMA1RST_R = crate::R<bool, DMA1RST_A>;
impl DMA1RST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, DMA1RST_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(DMA1RST_A::RESET),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DMA1RST_A::RESET
    }
}
#[doc = "Write proxy for field `DMA1RST`"]
pub struct DMA1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
#[doc = "DMA2 reset"]
pub type DMA2RST_A = DMA1RST_A;
#[doc = "Reader of field `DMA2RST`"]
pub type DMA2RST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `DMA2RST`"]
pub struct DMA2RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA2RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
#[doc = "DMAMUXRST"]
pub type DMAMUX1RST_A = DMA1RST_A;
#[doc = "Reader of field `DMAMUX1RST`"]
pub type DMAMUX1RST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `DMAMUX1RST`"]
pub struct DMAMUX1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1RST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAMUX1RST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "CORDIC reset"]
pub type CORDICRST_A = DMA1RST_A;
#[doc = "Reader of field `CORDICRST`"]
pub type CORDICRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `CORDICRST`"]
pub struct CORDICRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CORDICRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CORDICRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "FMAC reset"]
pub type FMACRST_A = DMA1RST_A;
#[doc = "Reader of field `FMACRST`"]
pub type FMACRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `FMACRST`"]
pub struct FMACRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FMACRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMACRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Flash memory interface reset"]
pub type FLASHRST_A = DMA1RST_A;
#[doc = "Reader of field `FLASHRST`"]
pub type FLASHRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `FLASHRST`"]
pub struct FLASHRST_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASHRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "CRC reset"]
pub type CRCRST_A = DMA1RST_A;
#[doc = "Reader of field `CRCRST`"]
pub type CRCRST_R = crate::R<bool, DMA1RST_A>;
#[doc = "Write proxy for field `CRCRST`"]
pub struct CRCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CRCRST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(DMA1RST_A::RESET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamux1rst(&self) -> DMAMUX1RST_R {
        DMAMUX1RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CORDIC reset"]
    #[inline(always)]
    pub fn cordicrst(&self) -> CORDICRST_R {
        CORDICRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FMAC reset"]
    #[inline(always)]
    pub fn fmacrst(&self) -> FMACRST_R {
        FMACRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> DMA1RST_W {
        DMA1RST_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> DMA2RST_W {
        DMA2RST_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUXRST"]
    #[inline(always)]
    pub fn dmamux1rst(&mut self) -> DMAMUX1RST_W {
        DMAMUX1RST_W { w: self }
    }
    #[doc = "Bit 3 - CORDIC reset"]
    #[inline(always)]
    pub fn cordicrst(&mut self) -> CORDICRST_W {
        CORDICRST_W { w: self }
    }
    #[doc = "Bit 4 - FMAC reset"]
    #[inline(always)]
    pub fn fmacrst(&mut self) -> FMACRST_W {
        FMACRST_W { w: self }
    }
    #[doc = "Bit 8 - Flash memory interface reset"]
    #[inline(always)]
    pub fn flashrst(&mut self) -> FLASHRST_W {
        FLASHRST_W { w: self }
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W {
        CRCRST_W { w: self }
    }
}
