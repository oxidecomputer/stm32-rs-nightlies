#[doc = "Reader of register SPDIFRX_CR"]
pub type R = crate::R<u32, super::SPDIFRX_CR>;
#[doc = "Writer for register SPDIFRX_CR"]
pub type W = crate::W<u32, super::SPDIFRX_CR>;
#[doc = "Register SPDIFRX_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::SPDIFRX_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPDIFRXEN`"]
pub type SPDIFRXEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SPDIFRXEN`"]
pub struct SPDIFRXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFRXEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `RXDMAEN`"]
pub type RXDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMAEN`"]
pub struct RXDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAEN_W<'a> {
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
#[doc = "Reader of field `RXSTEO`"]
pub type RXSTEO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXSTEO`"]
pub struct RXSTEO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTEO_W<'a> {
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
#[doc = "Reader of field `DRFMT`"]
pub type DRFMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DRFMT`"]
pub struct DRFMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DRFMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `PMSK`"]
pub type PMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMSK`"]
pub struct PMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `VMSK`"]
pub type VMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VMSK`"]
pub struct VMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> VMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `CUMSK`"]
pub type CUMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CUMSK`"]
pub struct CUMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> CUMSK_W<'a> {
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
#[doc = "Reader of field `PTMSK`"]
pub type PTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTMSK`"]
pub struct PTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> PTMSK_W<'a> {
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
#[doc = "Reader of field `CBDMAEN`"]
pub type CBDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBDMAEN`"]
pub struct CBDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CBDMAEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `CHSEL`"]
pub type CHSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHSEL`"]
pub struct CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `NBTR`"]
pub type NBTR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBTR`"]
pub struct NBTR_W<'a> {
    w: &'a mut W,
}
impl<'a> NBTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `WFA`"]
pub type WFA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WFA`"]
pub struct WFA_W<'a> {
    w: &'a mut W,
}
impl<'a> WFA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `INSEL`"]
pub type INSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSEL`"]
pub struct INSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `CKSEN`"]
pub type CKSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSEN`"]
pub struct CKSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `CKSBKPEN`"]
pub type CKSBKPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CKSBKPEN`"]
pub struct CKSBKPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSBKPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SPDIFRXEN_R {
        SPDIFRXEN_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&self) -> RXSTEO_R {
        RXSTEO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&self) -> DRFMT_R {
        DRFMT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&self) -> PMSK_R {
        PMSK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&self) -> VMSK_R {
        VMSK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&self) -> CUMSK_R {
        CUMSK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&self) -> PTMSK_R {
        PTMSK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&self) -> CBDMAEN_R {
        CBDMAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&self) -> NBTR_R {
        NBTR_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&self) -> WFA_R {
        WFA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&self) -> INSEL_R {
        INSEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&self) -> CKSEN_R {
        CKSEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&self) -> CKSBKPEN_R {
        CKSBKPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPDIFRXEN"]
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SPDIFRXEN_W {
        SPDIFRXEN_W { w: self }
    }
    #[doc = "Bit 2 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W {
        RXDMAEN_W { w: self }
    }
    #[doc = "Bit 3 - RXSTEO"]
    #[inline(always)]
    pub fn rxsteo(&mut self) -> RXSTEO_W {
        RXSTEO_W { w: self }
    }
    #[doc = "Bits 4:5 - DRFMT"]
    #[inline(always)]
    pub fn drfmt(&mut self) -> DRFMT_W {
        DRFMT_W { w: self }
    }
    #[doc = "Bit 6 - PMSK"]
    #[inline(always)]
    pub fn pmsk(&mut self) -> PMSK_W {
        PMSK_W { w: self }
    }
    #[doc = "Bit 7 - VMSK"]
    #[inline(always)]
    pub fn vmsk(&mut self) -> VMSK_W {
        VMSK_W { w: self }
    }
    #[doc = "Bit 8 - CUMSK"]
    #[inline(always)]
    pub fn cumsk(&mut self) -> CUMSK_W {
        CUMSK_W { w: self }
    }
    #[doc = "Bit 9 - PTMSK"]
    #[inline(always)]
    pub fn ptmsk(&mut self) -> PTMSK_W {
        PTMSK_W { w: self }
    }
    #[doc = "Bit 10 - CBDMAEN"]
    #[inline(always)]
    pub fn cbdmaen(&mut self) -> CBDMAEN_W {
        CBDMAEN_W { w: self }
    }
    #[doc = "Bit 11 - CHSEL"]
    #[inline(always)]
    pub fn chsel(&mut self) -> CHSEL_W {
        CHSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - NBTR"]
    #[inline(always)]
    pub fn nbtr(&mut self) -> NBTR_W {
        NBTR_W { w: self }
    }
    #[doc = "Bit 14 - WFA"]
    #[inline(always)]
    pub fn wfa(&mut self) -> WFA_W {
        WFA_W { w: self }
    }
    #[doc = "Bits 16:18 - INSEL"]
    #[inline(always)]
    pub fn insel(&mut self) -> INSEL_W {
        INSEL_W { w: self }
    }
    #[doc = "Bit 20 - CKSEN"]
    #[inline(always)]
    pub fn cksen(&mut self) -> CKSEN_W {
        CKSEN_W { w: self }
    }
    #[doc = "Bit 21 - CKSBKPEN"]
    #[inline(always)]
    pub fn cksbkpen(&mut self) -> CKSBKPEN_W {
        CKSBKPEN_W { w: self }
    }
}
