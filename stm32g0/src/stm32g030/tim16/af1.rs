#[doc = "Reader of register AF1"]
pub type R = crate::R<u32, super::AF1>;
#[doc = "Writer for register AF1"]
pub type W = crate::W<u32, super::AF1>;
#[doc = "Register AF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::AF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BKINE`"]
pub type BKINE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKINE`"]
pub struct BKINE_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINE_W<'a> {
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
#[doc = "Reader of field `BKCMP1E`"]
pub type BKCMP1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKCMP1E`"]
pub struct BKCMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1E_W<'a> {
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
#[doc = "Reader of field `BKCMP2E`"]
pub type BKCMP2E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKCMP2E`"]
pub struct BKCMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2E_W<'a> {
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
#[doc = "Reader of field `BKDFBK1E`"]
pub type BKDFBK1E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKDFBK1E`"]
pub struct BKDFBK1E_W<'a> {
    w: &'a mut W,
}
impl<'a> BKDFBK1E_W<'a> {
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
#[doc = "Reader of field `BKINP`"]
pub type BKINP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKINP`"]
pub struct BKINP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKINP_W<'a> {
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
#[doc = "Reader of field `BKCMP1P`"]
pub type BKCMP1P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKCMP1P`"]
pub struct BKCMP1P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP1P_W<'a> {
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
#[doc = "Reader of field `BKCMP2P`"]
pub type BKCMP2P_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKCMP2P`"]
pub struct BKCMP2P_W<'a> {
    w: &'a mut W,
}
impl<'a> BKCMP2P_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&self) -> BKDFBK1E_R {
        BKDFBK1E_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BKINE_W {
        BKINE_W { w: self }
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W {
        BKCMP1E_W { w: self }
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W {
        BKCMP2E_W { w: self }
    }
    #[doc = "Bit 8 - BRK DFSDM_BREAK1 enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&mut self) -> BKDFBK1E_W {
        BKDFBK1E_W { w: self }
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BKINP_W {
        BKINP_W { w: self }
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W {
        BKCMP1P_W { w: self }
    }
    #[doc = "Bit 11 - BRK COMP2 input polarit"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W {
        BKCMP2P_W { w: self }
    }
}
